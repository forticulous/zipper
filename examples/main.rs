extern crate zipper;

use std::env;
use std::path::Path;

use zipper::Archive;

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let path: &Path = Path::new(&filename);

    let mut archive = Archive::new(path).expect("Failed to open archive");
    //archive.print_info().expect("print_info panic");
    archive.unzip().expect("unzip panic");
}

