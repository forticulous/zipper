extern crate zipper;

use std::env;
use std::path::Path;

use zipper::{Archive, EndOfCentralDirectory, CentralDirectoryFileHeader};

fn main() {
  let filename = env::args().nth(1).expect("Missing filename argument");
  let path: &Path = Path::new(&filename);

  let mut archive = Archive::new(path).expect("Failed to open archive");

  let vec_cdfh: Vec<CentralDirectoryFileHeader> = archive.cd_iter().unwrap().collect();

  for cdfh in vec_cdfh {
    println!("{}\n", cdfh);

    let lfh_start = cdfh.local_file_header_start;
    let lfh = archive.read_lfh(lfh_start).expect("Failed to read lfh");
    println!("{}\n", lfh);
  }

  let eocd: EndOfCentralDirectory = archive.read_eocd().expect("Failed to get EOCD");
  println!("{}\n", eocd);
}
