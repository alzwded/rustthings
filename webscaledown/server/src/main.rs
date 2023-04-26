mod resolution;
mod imageproc;

use actix_web::{
    web,
    App,
    HttpServer,
    Responder,
    HttpRequest,
    HttpResponse,
    Error,
    middleware::Logger,
};
use actix_multipart::form::{
    text::Text,
    //bytes::Bytes,
    tempfile::TempFile,
    MultipartForm,
};
use log::*;
use image::{
    codecs::jpeg::JpegEncoder,
};
use crate::resolution::Resolution;

async fn greet(req: HttpRequest) -> impl Responder {
    if let Some(topic) = req.match_info().get("topic") {
        match topic {
            "help" => format!("GET /help/\nGET /help/{topic}\nLists endpoints"),
            "downscale" => format!("POST /downscale\nbody: JPEG\nresponse: JPEG"),
            _ => format!("invalid"), // TODO bad request
        }
    } else {
        format!("Topics:\ndownscale\nhelp")
    }
}

#[derive(MultipartForm)]
struct DownscaleForm {
    #[multipart(limit = "64 B")]
    pub resolution: Text<String>,
    #[multipart(limit = "25 MiB")]
    pub image: TempFile,
}

async fn downscale(form: MultipartForm<DownscaleForm>) -> Result<HttpResponse, Error> {
    //let bytes = web::Bytes::extract(&req).wait().unwrap();
    debug!("entered");
    debug!("Form: resolution {:?} image content-type {:?} size {} file_name {:?}", form.resolution, form.image.content_type, form.image.size, form.image.file_name);
    match &form.image.content_type {
        Some(m) => match (m.type_(), m.subtype()) {
            (mime::IMAGE, _) => (),
            _ => {
                error!("Not an image: {:?}", m);
                return Ok(HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST).body("Expected an image"));
            },
        },
        None => {
                error!("Not an image");
            return Ok(HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST).body("Expected an image"))
        },
    }

    let oresolution = Resolution::from_string(&form.resolution.0);
    match oresolution {
        Err(e) => return Ok(HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST).body(format!("Bad resolution argument: {}", e))),
        _ => (),
    }
    let target_resolution = oresolution.unwrap();

    let image;
    debug!("Temp file path: {}", form.image.file.path().to_str().unwrap());
    match imageproc::read_image(form.image.file.path().to_str().unwrap()) {
        Ok(x) => {
            image = x;
        },
        Err(e) => return Ok(HttpResponse::build(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR).body(format!("{}", e))),
    }

    let original_resolution = resolution::Resolution::new(image.width(), image.height());

    if target_resolution >= original_resolution {
        return Ok(HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST).body(format!("{} is not smaller than {}, nothing to do", original_resolution, target_resolution)));
    }

    let new_resolution = original_resolution.scale_to(&target_resolution);
    
    match imageproc::downscale(&image, &new_resolution) {
        Ok(x) => {
            let mut bytes = Vec::new();
            let mut encoder = JpegEncoder::new(&mut bytes);
            if let Err(e) = encoder.encode_image(&x) {
                return Ok(HttpResponse::build(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR).body(format!("Failed to encode jpeg: {}", e)));
            }
            return Ok(HttpResponse::Ok()
                .content_type("image/jpeg")
                .body(bytes)
                );
        },
        Err(e) => {
            return Ok(HttpResponse::build(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR).body(format!("Failed to downsize image: {}", e)));
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .route("/", web::get().to(greet))
        .route("/help", web::get().to(greet))
        .route("/help/", web::get().to(greet))
        .route("/help/{topic}", web::get().to(greet))
        .route("/downscale", web::post().to(downscale))
    })
    .bind("0.0.0.0:8084")?
    .run()
    .await
}
