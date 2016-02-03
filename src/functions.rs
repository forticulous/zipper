use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, SeekFrom, Error, ErrorKind};
use std::mem;
use std::slice;
use std::path::Path;

extern crate flate2;
use self::flate2::{Decompress, Flush};

use structures::{CentralDirectoryFileHeader, EndOfCentralDirectory, LocalFileHeader};

enum ArchiveStructure {
  CentralDirectoryFileHeader,
  EndOfCentralDirectory,
  LocalFileHeader
}

fn constant_size_of(part: ArchiveStructure) -> usize {
  match part {
    ArchiveStructure::CentralDirectoryFileHeader => 46,
    ArchiveStructure::EndOfCentralDirectory      => 22,
    ArchiveStructure::LocalFileHeader            => 30
  }
}

pub fn open_file(filename: &String) -> io::Result<File> {
  let path: &Path = Path::new(filename);  
  let file = try!(File::open(path));
  Ok(file)
}

pub fn read_string(file: &mut File, len: usize) -> io::Result<String> {
  let mut v: Vec<u8> = Vec::with_capacity(len);
  for _ in 0..len { v.push(0) }

  try!(file.read_exact(&mut v));

  let s = try!(String::from_utf8(v).map_err(|_| Error::new(ErrorKind::InvalidData, "Not valid UTF-8")));
  Ok(s)
}

pub fn read_cdfh(file: &mut File) -> io::Result<CentralDirectoryFileHeader> {
  let mut cdfh = CentralDirectoryFileHeader::new();

  {
    let slice: &mut [u8] = unsafe { 
      slice::from_raw_parts_mut(&mut cdfh as *mut _ as *mut u8, constant_size_of(ArchiveStructure::CentralDirectoryFileHeader)) 
    };
    try!(file.read_exact(slice));
  }
  if cdfh.file_name_len > 0 {
    cdfh.file_name = try!(read_string(file, cdfh.file_name_len as usize));
  }
  if cdfh.extra_field_len > 0 {
    // TODO: Skip for now
    try!(file.seek(SeekFrom::Current(cdfh.extra_field_len as i64)));
  }
  if cdfh.comment_len > 0 {
    // TODO: Skip for now
    try!(file.seek(SeekFrom::Current(cdfh.comment_len as i64)));
  }

  Ok(cdfh)
}

pub fn read_eocd(file: &mut File) -> io::Result<EndOfCentralDirectory> {
  let mut eocd = EndOfCentralDirectory::new(); 
  
  let eocd_start = try!(find_sig_position(file, eocd.sig));
  try!(file.seek(SeekFrom::Start(eocd_start)));

  let slice: &mut [u8] = unsafe { 
    slice::from_raw_parts_mut(&mut eocd as *mut _ as *mut u8, constant_size_of(ArchiveStructure::EndOfCentralDirectory))
  };
  try!(file.read_exact(slice));

  Ok(eocd)
}

pub fn read_lfh(file: &mut File, lfh_start: u32) -> io::Result<LocalFileHeader> {
  let mut lfh = LocalFileHeader::new();

  try!(file.seek(SeekFrom::Start(lfh_start as u64)));

  {
    let slice: &mut [u8] = unsafe {
      slice::from_raw_parts_mut(&mut lfh as *mut _ as *mut u8, constant_size_of(ArchiveStructure::LocalFileHeader))
    };
    try!(file.read_exact(slice));
  }
  if lfh.file_name_len > 0 {
    lfh.file_name = try!(read_string(file, lfh.file_name_len as usize));
  }
  if lfh.extra_field_len > 0 {
    // TODO: Skip for now
    try!(file.seek(SeekFrom::Current(lfh.extra_field_len as i64)));
  }

  Ok(lfh)
}

pub fn read_lfh_raw_data(file: &mut File, cdfh: &CentralDirectoryFileHeader) -> io::Result<Vec<u8>> {
  let lfh_data_start = cdfh.local_file_header_start as usize +
    constant_size_of(ArchiveStructure::LocalFileHeader) +
    cdfh.file_name_len as usize +
    cdfh.extra_field_len as usize;
  let data_len = cdfh.compressed_size as usize;

  try!(file.seek(SeekFrom::Start(lfh_data_start as u64)));

  let mut buf_read = BufReader::with_capacity(data_len, file);
  let bytes: Vec<u8> = try!(buf_read.fill_buf()).to_vec();

  Ok(bytes)
}

pub fn uncompress_file_data(compressed: Vec<u8>, compression_method: u16) -> io::Result<Vec<u8>> {
  if compression_method != 8u16 {
    return Err(Error::new(ErrorKind::Other, "Unsupported compression method"));
  }

  let mut decompressed: Vec<u8> = Vec::with_capacity(compressed.len());
  let mut decomp = Decompress::new(false);

  decomp.decompress_vec(&compressed[..], &mut decompressed, Flush::Finish).expect("Failed to decompress file data");

  Ok(decompressed)
}

pub fn find_sig_position<T: Seek + Read>(source: &mut T, sig: u32) -> io::Result<u64> {
  try!(source.seek(SeekFrom::Start(0)));

  let mut buf = [0u8; 4];
  loop {
    try!(source.read_exact(&mut buf));
    let parsed: u32 = unsafe { mem::transmute(buf) };
    if parsed == sig {
      return Ok(try!(source.seek(SeekFrom::Current(-4))));
    }
    try!(source.seek(SeekFrom::Current(-3)));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;

  #[test]
  fn get_file() {
    let result = open_file(&String::from("archive.zip"));
    assert!(result.is_ok());
  }

  #[test]
  fn find_sig_eocd() {
    let bytes: &[u8] = &[0, 0, 0x50, 0x4b, 0x05, 0x06, 0, 0];  
    let mut cursor = Cursor::new(bytes);

    let res = find_sig_position(&mut cursor, EOCD_SIG);
    assert!(res.is_ok());
    assert_eq!(2u64, res.unwrap());
  }

}
