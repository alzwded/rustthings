// why dis needed? Shrugs; it was in the example
extern crate getopt;

use getopt::Opt;
use std::env;
use std::collections::hash_map::HashMap;
use std::process::exit;

fn print_version() {
    // the Rust runtime (cargo runtime?) magically injects
    // the version via environment variable
    println!("version {}", env!("CARGO_PKG_VERSION"));
    exit(1);
}

// we return maybe a tuple, maybe a getopt::Error
// I guess Result and Option are kinda like "maybe"?
pub fn parse_args() -> Result<(Vec<String>, HashMap<&'static str,Option<String>>), getopt::Error> {
    let mut resolution_s : Option<String> = None;
    const OPTSTRING: &'static str  = "r:V";
    let args: Vec<String> = env::args().collect();
    let mut opts = getopt::Parser::new(&args, OPTSTRING);
    
    loop {
        // the ? is magic; it auto returns the error if it contains an error
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('r', Some(arg)) => {
                    resolution_s = Some(arg.clone());
                },
                Opt('V', None) => {
                    print_version()
                },
                _ => unreachable!(),
            },
        }
    }

    // syntax to build a map from known values
    let map = HashMap::from([
        ("resolution", resolution_s)
    ]);

    // what is to_owned()? It created Owned data from Borrowed data, usually
    // via a clone; here in particular, we'd like to pass ownership of some
    // stuff back to the caller, but the local stuff will get kill'd off once
    // we exit the scope
    return Ok(( args.to_owned().split_off(opts.index()), map.to_owned() ));
}

