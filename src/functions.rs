use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Cursor, SeekFrom, Error, ErrorKind};
use std::mem;
use std::slice;
use std::path::Path;

extern crate flate2;
use self::flate2::read::DeflateDecoder;

use structures::{CentralDirectoryFileHeader, EndOfCentralDirectory, LocalFileHeader};

enum ArchiveStructure {
  CentralDirectoryFileHeader,
  EndOfCentralDirectory,
  LocalFileHeader
}

impl ArchiveStructure {
  fn constant_size_of(&self) -> usize {
    match *self {
      ArchiveStructure::CentralDirectoryFileHeader => 46,
      ArchiveStructure::EndOfCentralDirectory      => 22,
      ArchiveStructure::LocalFileHeader            => 30
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum CompressionMethod {
  Store,
  Deflate
}

impl CompressionMethod {
  pub fn from_code(code: u16) -> Option<CompressionMethod> {
    match code {
      0 => Some(CompressionMethod::Store),
      8 => Some(CompressionMethod::Deflate),
      _ => None
    }
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
      slice::from_raw_parts_mut(&mut cdfh as *mut _ as *mut u8, ArchiveStructure::CentralDirectoryFileHeader.constant_size_of()) 
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
    slice::from_raw_parts_mut(&mut eocd as *mut _ as *mut u8, ArchiveStructure::EndOfCentralDirectory.constant_size_of())
  };
  try!(file.read_exact(slice));

  Ok(eocd)
}

pub fn read_lfh(file: &mut File, lfh_start: u32) -> io::Result<LocalFileHeader> {
  let mut lfh = LocalFileHeader::new();

  try!(file.seek(SeekFrom::Start(lfh_start as u64)));

  {
    let slice: &mut [u8] = unsafe {
      slice::from_raw_parts_mut(&mut lfh as *mut _ as *mut u8, ArchiveStructure::LocalFileHeader.constant_size_of())
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
    ArchiveStructure::LocalFileHeader.constant_size_of() +
    cdfh.file_name_len as usize +
    cdfh.extra_field_len as usize;
  let data_len = cdfh.compressed_size as usize;

  try!(file.seek(SeekFrom::Start(lfh_data_start as u64)));

  let mut buf_read = BufReader::with_capacity(data_len, file);
  let bytes: Vec<u8> = try!(buf_read.fill_buf()).to_vec();

  Ok(bytes)
}

pub fn decompress_file_data(compressed: Vec<u8>, method: CompressionMethod) -> io::Result<Vec<u8>> {
  if CompressionMethod::Deflate != method {
    return Err(Error::new(ErrorKind::Other, "Unsupported compression method"));
  }

  let mut decompressed: Vec<u8> = Vec::with_capacity(compressed.len());
  let compressed_cursor = Cursor::new(compressed);
  let mut decoder = DeflateDecoder::new(compressed_cursor);

  let result = decoder.read_to_end(&mut decompressed);

  if result.is_err() {
    println!("\n\n{:?}\n\n", result.err().unwrap());
  }

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
  use std::io::prelude::*;
  use structures::Signature;

  extern crate flate2;
  use self::flate2::Compression;
  use self::flate2::write::ZlibEncoder;

  #[test]
  fn get_file() {
    let result = open_file(&String::from("archive.zip"));
    assert!(result.is_ok());
  }

  #[test]
  fn find_sig_eocd() {
    let bytes: &[u8] = &[0, 0, 0x50, 0x4b, 0x05, 0x06, 0, 0];  
    let mut cursor = Cursor::new(bytes);

    let res = find_sig_position(&mut cursor, Signature::EndOfCentralDirectory.sig_byte());
    assert!(res.is_ok());
    assert_eq!(2u64, res.unwrap());
  }

  #[test]
  fn compress() {
    let bytes: &[u8] = b"derp\x0a";
    assert_eq!(5, bytes.len());

    let expected: &[u8] = b"\xea\x9a\x5a\xf6\x4b\x49\x2d\x2a\xe0\x02\x00";

    let mut actual: Vec<u8> = Vec::new();
    {
      let mut compressor = ZlibEncoder::new(actual, Compression::Default);
      compressor.write_all(bytes).unwrap();
      actual = compressor.finish().unwrap();
    }

    println!("{:?}", bytes);
    println!("{:?}", expected);
    println!("{:?}", actual);
    assert_eq!(expected.len(), actual.len());
    assert_eq!(expected, &actual[..]);
  }

  //#[test]
  //fn decompress() {
  //  let bytes: &[u8] = b"herp\x0a";
  //  let mut encoder = DeflateEncoder::new(bytes, Compression::Default);
  //  let mut compressed: Vec<u8> = Vec::new();

  //  encoder.read_to_end(&mut compressed).unwrap();
  //  let decompressed = decompress_file_data(compressed, CompressionMethod::Deflate).unwrap();
  //  
  //  assert_eq!(bytes, &decompressed[..]);
  //}

  #[test]
  fn deflate_from_code() {
    let method = CompressionMethod::from_code(8u16).unwrap();
    assert_eq!(CompressionMethod::Deflate, method);
  }

}
