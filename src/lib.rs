use std::fs::File;
use std::io::prelude::*;
use std::io::{self, SeekFrom, Error, ErrorKind};
use std::mem;
use std::slice;
use std::path::Path;

pub mod structures;
use structures::{EndOfCentralDirectory, CentralDirectoryFileHeader};

pub fn unzip(filename: &String) -> io::Result<()> {
  Ok(())
}

pub fn open_file(filename: &String) -> io::Result<File> {
  let path: &Path = Path::new(filename);  
  let file = try!(File::open(path));
  Ok(file)
}

pub fn read_str(file: &mut File, len: usize) -> io::Result<String> {
  let mut v: Vec<u8> = Vec::with_capacity(len);
  // Safe because it was allocated with sufficient capacity
  unsafe { v.set_len(len) };
  let mut boxed: Box<[u8]> = v.into_boxed_slice();
  
  try!(file.read_exact(boxed.as_mut()));

  let s = String::from_utf8(boxed.into_vec()).expect("Not valid UTF-8");
  Ok(s)
}

pub fn get_cdfh(file: &mut File) -> io::Result<CentralDirectoryFileHeader> {
  let mut cdfh = CentralDirectoryFileHeader::new();

  let cdfh_start = try!(find_sig_position(file, cdfh.sig));
  try!(file.seek(SeekFrom::Start(cdfh_start)));

  let slice: &mut [u8] = unsafe { slice::from_raw_parts_mut(&mut cdfh as *mut _ as *mut u8, mem::size_of_val(&cdfh)) };
  try!(file.read_exact(slice));

  Ok(cdfh)
}

pub fn get_eocd(file: &mut File) -> io::Result<EndOfCentralDirectory> {
  let mut eocd = EndOfCentralDirectory::new(); 
  
  let eocd_start = try!(find_sig_position(file, eocd.sig));
  try!(file.seek(SeekFrom::Start(eocd_start)));

  let slice: &mut [u8] = unsafe { slice::from_raw_parts_mut(&mut eocd as *mut _ as *mut u8, mem::size_of_val(&eocd)) };
  try!(file.read_exact(slice));

  Ok(eocd)
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

pub fn get_file_size(file: &File) -> io::Result<u64> {
  let metadata = try!(file.metadata());
  Ok(metadata.len())
}

#[cfg(test)]
mod test {
  use super::*;
  use std::io::Cursor;

  #[test]
  fn unzip_test() {
    let result = unzip(&String::from("archive.zip"));
    assert!(result.is_ok());
  }

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
