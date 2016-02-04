use std::fs::File;

use functions::read_cdfh;
use cdfh::CentralDirectoryFileHeader;

pub struct CentralDirectoryIter<'a> {
    pub file: &'a mut File
}

impl<'a> Iterator for CentralDirectoryIter<'a> {
    type Item = CentralDirectoryFileHeader;

    fn next(&mut self) -> Option<CentralDirectoryFileHeader> {
        read_cdfh(self.file).ok()
    }
}
