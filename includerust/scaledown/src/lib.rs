mod resolution;
mod imageproc;

use std::ffi::c_char;
use std::ffi::CStr;
use crate::resolution::Resolution;

pub const DEFAULT_OUT_EXTENSION: &'static str = "out.jpg";

// we can't initialize a String, so this is an Option<>;
// we'll make sure it's intialized later
static mut ERRSTR: Option<String> = None;

unsafe fn set_errstr(s: String) {
    ERRSTR = Some(s);
}

// if we wanted to return a free-able string... don't; the allocators
// will probably be mismatched; it's always best to use caller allocated
// memory
/// resturns a static string; do not free
#[no_mangle]
pub unsafe extern "C" fn errstr() -> *const c_char {
    match &ERRSTR {
        None => {
            ERRSTR = Some(String::from(""));
        },
        _ => (),
    }
    match &ERRSTR {
        Some(s) => s.as_ptr() as *const c_char,
        _ => unreachable!(),
    }
}

// the C wrapper, where we deal with unsafe code
#[no_mangle]
pub unsafe extern "C" fn convertw(
        cinpath: *const c_char,
        coutpath: *const c_char,
        cresolution: *const Resolution)
-> i32 {
    if cinpath == std::ptr::null() {
        set_errstr(String::from("nul cinpath"));
        return -1;
    }
    if cresolution == std::ptr::null() {
        set_errstr(String::from("nul cresolution"));
        return -1;
    }
    let inpath = String::from_utf8_lossy(CStr::from_ptr(cinpath).to_bytes()).to_string();
    let outpath = if coutpath == std::ptr::null() {
        None
    } else {
        Some(String::from_utf8_lossy(CStr::from_ptr(coutpath).to_bytes()).to_string())
    };
    let resolution: &Resolution = &*cresolution;
    // run the actual rust code
    return match convert(inpath, outpath, resolution) {
        Ok(()) => 0,
        Err(e) => {
            // on error, save the error string in case the caller wants to fetch it
            set_errstr(String::from(e));
            -1
        }
    }
}

// the safe rust code
pub fn convert(
        inpath: String,
        maybeoutpath: Option<String>,
        target_resolution: &resolution::Resolution
) -> Result<(), String> {
    let image;
    match imageproc::read_image(&inpath) {
        Ok(x) => {
            image = x;
        },
        Err(e) => {
            return Err(format!("{}", e));
        },
    }
    let original_resolution = resolution::Resolution::new(image.width(), image.height());
    let new_resolution = original_resolution.scale_to(&target_resolution);
    if new_resolution <= original_resolution {
        let outpath = match maybeoutpath {
            Some(s) => s,
            None => {
                let ppath = std::path::Path::new(&inpath);
                String::from(ppath.with_extension(DEFAULT_OUT_EXTENSION).to_str().unwrap())
            },
        };
        println!("downscaling {}", inpath);
        match imageproc::downscale(&image, &new_resolution) {
            Ok(x) => match imageproc::write_image(&x, &outpath) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to write {}: {}", &outpath, e)),
            },
            Err(e) => Err(format!("{} failed to downsize: {}", &inpath, e)),
        }
    } else {
        Err(format!("Target resolution {} is too big for source resolution {}", target_resolution, original_resolution))
    }
}
