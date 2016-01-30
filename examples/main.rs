extern crate zipper;

use std::env;
use std::fs::File;

use zipper::{EndOfCentralDirectory, CentralDirectoryFileHeader, CentralDirectoryIter};

fn main() {
  let filename = env::args().nth(1).expect("Missing filename argument");
  let mut file: File = zipper::open_file(&filename).expect("Couldn't open file");

  let eocd_start: u64 = zipper::find_sig_position(&mut file, zipper::EOCD_SIG).unwrap();
  println!("EOCD starts {}", eocd_start); 

  let eocd: EndOfCentralDirectory = zipper::get_eocd(&mut file).expect("Failed to get EOCD");
  println!("{:08x}", eocd.sig);
  println!("{:?}", eocd);

  let mut iter = CentralDirectoryIter::new(&mut file);
  for (cdfh, filename) in iter {
    println!("- {:08x}", cdfh.sig);
    println!("- {:?}", cdfh);
    println!("- {}", filename);
  }
}
