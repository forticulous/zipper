extern crate zipper;

use std::env;
use std::fs::File;

use zipper::structures::{EndOfCentralDirectory, CentralDirectoryFileHeader};

fn main() {
  let filename = env::args().nth(1).expect("Missing filename argument");
  let mut file: File = zipper::open_file(&filename).expect("Couldn't open file");

  let eocd: EndOfCentralDirectory = zipper::get_eocd(&mut file).expect("Failed to get EOCD");
  println!("{:08x}", eocd.sig);
  println!("{:?}", eocd);

  let cdfh: CentralDirectoryFileHeader = zipper::get_cdfh(&mut file).expect("Failed to get CDFH");
  println!("{:08x}", cdfh.sig);
  println!("{:?}", cdfh);

  let filename: String = zipper::read_str(&mut file, cdfh.file_name_len as usize).expect("Failed to read filename");
  println!("{}", filename);
}
