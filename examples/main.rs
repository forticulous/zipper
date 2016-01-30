extern crate zipper;

use std::env;
use std::path::Path;

use zipper::{Archive, EndOfCentralDirectory};

fn main() {
  let filename = env::args().nth(1).expect("Missing filename argument");
  let path: &Path = Path::new(&filename);

  let mut archive = Archive::new(path).expect("Failed to open archive");

  let eocd: EndOfCentralDirectory = archive.read_eocd().expect("Failed to get EOCD");
  println!("{:08x}", eocd.sig);
  println!("{:?}\n", eocd);

  let iter = archive.cd_iter().expect("Failed to get CD iter");
  for (cdfh, filename) in iter {
    println!("- {:08x}", cdfh.sig);
    println!("- {:?}", cdfh);
    println!("- {}\n", filename);
  }
}
