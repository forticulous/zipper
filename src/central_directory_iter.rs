use std::fs::File;

use functions::read_cdfh;
use cdfh::CentralDirectoryFileHeader;

pub struct CentralDirectoryIter<'a> {
  pub file: &'a mut File
}

impl<'a> Iterator for CentralDirectoryIter<'a> {
  type Item = CentralDirectoryFileHeader;

  fn next(&mut self) -> Option<CentralDirectoryFileHeader> {
    let result = read_cdfh(self.file);
    if result.is_err() {
        return None;
    }

    let cdfh = result.unwrap();
    Some(cdfh)
  }
}
