// declare module
mod cstuff;

// use function from module
use crate::cstuff::GetLibcVersion;

fn main() {
    // call safe wrapper
    let version = match GetLibcVersion() {
        Ok(s) => s,
        Err(s) => String::from(s),
    };
    println!("Version is {}", version);
}
