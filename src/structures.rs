use std::fs::File;
use std::io::{self, SeekFrom};
use std::io::prelude::*;
use std::path::Path;

use functions::{read_cdfh, read_eocd};

// Constants
pub const EOCD_SIG: u32 = 0x06054b50;
pub const CDFH_SIG: u32 = 0x02014b50;

pub struct Archive {
  file: File
}

impl Archive {
  pub fn new(path: &Path) -> io::Result<Archive> {
    let file = try!(File::open(path));
    
    let archive = Archive {
      file: file
    };
    Ok(archive)
  }

  pub fn read_eocd(&mut self) -> io::Result<EndOfCentralDirectory> {
    read_eocd(&mut self.file)
  }
 
  pub fn cd_iter(&mut self) -> io::Result<CentralDirectoryIter> {
    let eocd = try!(self.read_eocd());
    let cdfh_start: u32 = eocd.cd_start_offset;
    self.file.seek(SeekFrom::Start(cdfh_start as u64)).expect("Failed to seek");

    let iter = CentralDirectoryIter {
      file: &mut self.file
    };
    Ok(iter)
  }
}

pub struct CentralDirectoryIter<'a> {
  file: &'a mut File
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

#[repr(packed)]
#[derive(Debug)]
pub struct CentralDirectoryFileHeader {
  pub sig: u32,
  pub created_ver: u16,
  pub extract_ver: u16,
  pub general_bit_flag: u16,
  pub compression_method: u16,
  pub last_modified_time: u16,
  pub last_modified_date: u16,
  pub crc_32: u32,
  pub compressed_size: u32,
  pub uncompressed_size: u32,
  pub file_name_len: u16,
  pub extra_field_len: u16,
  pub comment_len: u16,
  pub file_start_disk_num: u16,
  pub internal_file_attr: u16,
  pub external_file_attr: u32,
  pub local_file_header_start: u32,
  pub file_name: String,
  pub extra_field: String,
  pub comment: String
}

impl CentralDirectoryFileHeader {
  pub fn new() -> CentralDirectoryFileHeader {
    CentralDirectoryFileHeader {
      sig: CDFH_SIG,
      created_ver: 0,
      extract_ver: 0,
      general_bit_flag: 0,
      compression_method: 0,
      last_modified_time: 0,
      last_modified_date: 0,
      crc_32: 0,
      compressed_size: 0,
      uncompressed_size: 0,
      file_name_len: 0,
      extra_field_len: 0,
      comment_len: 0,
      file_start_disk_num: 0,
      internal_file_attr: 0,
      external_file_attr: 0,
      local_file_header_start: 0,
      file_name: String::new(),
      extra_field: String::new(),
      comment: String::new()
    }
  }
}

#[repr(packed)]
#[derive(Debug)]
pub struct EndOfCentralDirectory {
  pub sig: u32,
  pub this_disk_num: u16,
  pub cd_start_disk: u16,
  pub cd_records_on_this_disk: u16,
  pub total_cd_records: u16,
  pub cd_size_bytes: u32,
  pub cd_start_offset: u32,
  pub comment_len: u16
}

impl EndOfCentralDirectory {
  pub fn new() -> EndOfCentralDirectory {
    EndOfCentralDirectory { 
      sig: EOCD_SIG, 
      this_disk_num: 0, 
      cd_start_disk: 0,
      cd_records_on_this_disk: 0,
      total_cd_records: 0,
      cd_size_bytes: 0,
      cd_start_offset: 0,
      comment_len: 0
    }
  }
}
