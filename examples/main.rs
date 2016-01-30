extern crate zipper;

use std::env;
use std::path::Path;

use zipper::{Archive, EndOfCentralDirectory};

fn main() {
  let filename = env::args().nth(1).expect("Missing filename argument");
  let path: &Path = Path::new(&filename);

  let mut archive = Archive::new(path).expect("Failed to open archive");

  {
    let iter = archive.cd_iter().expect("Failed to get CD iter");
    for cdfh in iter {
      println!("{}\n", cdfh);
    }
  }

  let eocd: EndOfCentralDirectory = archive.read_eocd().expect("Failed to get EOCD");
  println!("{}\n", eocd);
}
