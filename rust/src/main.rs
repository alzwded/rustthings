// Note: main.rs/lib.rs/mod.rs are special
//       mod xyz; statements here refer to files at the same dir level
//       elsewhere, it would look in a subdirectory whose name matches the file including
mod parse_args;
mod resolution;
mod imageproc;

// main is special, obviously; it's the executable's entry point
fn main() {
    let (files, opts) = match parse_args::parse_args() {
        Ok((vec, opts)) => (vec, opts),
        Err(error) => panic!("Bad args {}", error),
    };

    assert_ne!(files.len(), 0);

    for file in files {
        let path = String::from(&file);
        let image;
        match imageproc::read_image(&path) {
            Ok(x) => {
                image = x;
            },
            Err(e) => {
                println!("Failed to read {}: {}", &path, e);
                continue;
            }
        };
        let original_resolution = resolution::Resolution::new(image.width(), image.height());
        //println!("The image is {}", original_resolution);
        let target_resolution = resolution::Resolution::from_string(&match &opts["resolution"] {
            // shenanigans you would do in C++ to get both sides of ?: to return the same thing;
            // in this case, a String (not &String nor anything else)
            Some(s) => s.clone(),
            None => "800x600".to_string(),
        });
        //println!("Target Resolution: {}", target_resolution);
        //println!("Scaled to 800: {}", original_resolution.scale_to_square(800));
        let new_resolution = original_resolution.scale_to(&target_resolution);
        //println!("Scaled to target: {}", original_resolution.scale_to(&target_resolution));
        //println!("{} pixels", image.pixels().count());

        // NOTE: we had to implement PartialOrd::le for this operator to work
        if new_resolution <= original_resolution {
            //println!("Will reduce image from {} to {}", original_resolution, new_resolution);
            let ppath = std::path::Path::new(&path);
            // to_str() goes from PathBuf to ?a string?
            // unwrap() escapes out of Option<_>
            let outpath = String::from(ppath.with_extension("out.jpg").to_str().unwrap());
            // NOTE: variables can be intersplices in the format, i.e.
            //       println!("saving as {outpath}");
            //       ...which is a bit magical
            println!("saving as {}", outpath);
            match imageproc::downscale(&image, &new_resolution) {
                Ok(x) => if let Err(e) = imageproc::write_image(&x, &outpath) {
                    println!("Failed to write {}: {}", &outpath, e);
                },
                Err(e) => println!("{} failed to downsize: {}", &path, e),
            }
        } else {
            println!("Skipping {}", path);
        }
    }
}
