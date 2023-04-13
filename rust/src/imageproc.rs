//use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use num;

use rayon::prelude::*;

use crate::resolution;

pub fn read_image(path: &String) -> Result<DynamicImage, String> {
    println!("reading {}", path);
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode(){
            Ok(image) => Ok(image),
            Err(e) => Err(format!("{}", e)), // converting any sort of error to a stringy form for printing, I guess
        },
        Err(e) => Err(format!("{}", e))
    }
}

// 'a is a named format specifier
// obviously, 'static is special; there are probably other special ones
// Why dis needed? *shrugs*
// My guess is to clarify that whatever we put in iflat will live as long as
// this struct and not get deleted earlier; the docs kinda rhyme with that
struct WorkItem<'a, Buffer> {
    iflat: &'a image::FlatSamples<Buffer>,
    tx: u32,
    ty: u32,
    vw: f32,
    vh: f32,
}

// I really wanted to write some macros
macro_rules! skew {
    ($expression:expr) => {
        // NOTE: constants are in std::f32 (crate, NOT the fundamental type)
        //       but atan IS in the fundamental f32 type
        ((std::f32::consts::PI/2f32 - f32::atan($expression)) / (std::f32::consts::PI/2f32))
    };
}

macro_rules! sqr {
    ($expression:expr) => {
        // I strongly hope the compiler knows to not compute this twice,
        // because you can only have one expression in a macro
        $expression * $expression
    };
}

// I tried to make this generic with <T>, but failed
fn get_sample_f32(s: &Option<&u8>) -> f32
{
    match s {
        // what is **x? I think the first gets you &u8 and the second u8,
        // implying that originally you have a reference to a borrowed u8
        Some(x) => (**x) as f32,
        None => 0f32,
    }
}

pub fn downscale(img: &DynamicImage, new_res: &resolution::Resolution) -> Result<DynamicImage, String> {
    let vw: f32 = (img.width() as f32) / (new_res.width() as f32);
    let vh: f32 = (img.height() as f32) / (new_res.height() as f32);

    let rgb8 = img.to_rgb8();
    let iflat = rgb8.as_flat_samples();

    let olen = new_res.count();

    // let the shenanigans begin!
    // 1. we need to compute `olen' output pixels; so convert that
    //    range into an iterable thing;
    // 2. convert a number into a WorkItem
    // 3. ask Rayon to do SMP things to our iterable collection, and tell it how
    //    to chunk the data
    // 4. operate on a chunk, i.e. iterate over it and actually operate on a single
    //    WorkItem at a time
    // 5. flatten everything to a single array of byte values
    // 6. finally, create the image from these raw samples and return
    // This code could probably be genericized, but meh
    println!("num threads: {}", rayon::current_num_threads());
    let rawsamples: Vec<u8> = (0..olen as u32).into_iter()
            .map(|i| -> WorkItem<_> {
                WorkItem { 
                    iflat: &iflat, 
                    tx: i % new_res.width(),
                    ty: i / new_res.width(),
                    vw: vw,
                    vh: vh,
                }
            })
            .collect::<Vec<WorkItem<_>>>()
            .par_iter()
            //.chunks(olen / rayon::current_num_threads())
            .chunks(new_res.width() as usize) // this is faster than num_threads
            .map(|wis| -> Vec<u8> {
                wis.iter()
                .map(|wi| -> Vec<u8> {
                    let mut r = 0f32;
                    let mut g = 0f32;
                    let mut b = 0f32;
                    let mut s = 0f32;
                    let minh: i32 = ( (((wi.ty as i32) - 1) as f32) * wi.vh ) as i32;
                    let maxh: i32 = ( (((wi.ty as i32) + 1) as f32) * wi.vh ) as i32;
                    let minw: i32 = ( (((wi.tx as i32) - 1) as f32) * wi.vw ) as i32;
                    let maxw: i32 = ( (((wi.tx as i32) + 1) as f32) * wi.vw ) as i32;

                    for ii in minh..=maxh {
                        if ii < 0 || ii >= wi.iflat.layout.height as i32 {
                            continue;
                        }
                        for jj in minw..=maxw {
                            if jj < 0 || jj >= wi.iflat.layout.width as i32 {
                                continue;
                            }

                            let y = (vh * (wi.ty as f32)) - (ii as f32);
                            let x = (vw * (wi.tx as f32)) - (jj as f32);
                            let vv = (vh + vw) / 2f32;
                            let dist = sqr!(f32::abs( y * y + x * x ) / (vv * vv));
                            let skew = skew!(dist);

                            let ir = get_sample_f32(&wi.iflat.get_sample(0u8, jj as u32, ii as u32));
                            let ig = get_sample_f32(&wi.iflat.get_sample(1u8, jj as u32, ii as u32));
                            let ib = get_sample_f32(&wi.iflat.get_sample(2u8, jj as u32, ii as u32));

                            r = r + skew * ir;
                            g = g + skew * ig;
                            b = b + skew * ib;
                            s = s + skew;
                        }
                    }
                    
                    return vec![
                        num::clamp(r/s, 0f32, 255f32) as u8,
                        num::clamp(g/s, 0f32, 255f32) as u8,
                        num::clamp(b/s, 0f32, 255f32) as u8
                    ];
                })
                .flatten()
                .collect()
            })
            .flatten()
            .collect()
            ;

    match image::RgbImage::from_raw(new_res.width(), new_res.height(), rawsamples) {
        Some(image) => Ok(DynamicImage::ImageRgb8(image)),
        None => Err(String::from("Failed to get image")),
    }
}

pub fn write_image(img: &DynamicImage, path: &String) -> Result<(), String> {
    match img.save(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write to {}: {}", path, e)),
    }
}

