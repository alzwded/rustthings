// C code is non compliant, code standards-wise :-)
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// this macro includes all the bindings generated by bindgen
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;
use std::ffi::c_char;
use errno::errno;

// this is the sort of thing bindgen generates, but we can write them manually
extern "C" {
    // note: this is not exported out of the module, you need 'pub' for that
    fn dlerror() -> *const c_char;
}

// "safe" rust wrapper around unsafe C function
pub fn GetLibcVersion() -> Result<String, &'static str> {
    let mut cs: *const c_char = std::ptr::null();
    match unsafe { magic(&mut cs) } { // C code is unsafe
        0 => Ok(unsafe { String::from_utf8_lossy(CStr::from_ptr(cs).to_bytes()).to_string() }),
        _ => {
            let dlerror_s = unsafe { dlerror() }; // this may be null
            println!("dlerror: {}", unsafe { if dlerror_s == std::ptr::null() {
                String::from("<null>")
            } else {
                String::from_utf8_lossy(CStr::from_ptr( dlerror_s ).to_bytes()).to_string()
            } } );
            println!("errno: {}", errno()); // print strerror, which rust thankfully 
                                            // made a tuple that serializes by way
                                            // of strerror; the code itself is
                                            // errno().0
            Err("unknown")
        },
    }
}
