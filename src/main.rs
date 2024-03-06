mod lib;

use lib::*;

pub fn main() {
    println!("Version is {}", get_version());
    println!("Engine types are {}", get_engine_types());
}
