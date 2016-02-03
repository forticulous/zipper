use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{self, SeekFrom};
use std::io::prelude::*;
use std::path::Path;
use std::fmt;

use functions::{CompressionMethod, read_cdfh, read_eocd, read_lfh, read_lfh_raw_data, decompress_file_data};

#[derive(Debug)]
pub enum Signature {
  EndOfCentralDirectory,
  CentralDirectoryFileHeader,
  LocalFileHeader
}

impl Signature {
  pub fn sig_byte(&self) -> u32 {
    match *self {
      Signature::EndOfCentralDirectory      => 0x06054b50,
      Signature::CentralDirectoryFileHeader => 0x02014b50,
      Signature::LocalFileHeader            => 0x04034b50
    }
  }
}

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
 
  pub fn read_lfh(&mut self, lfh_start: u32) -> io::Result<LocalFileHeader> {
    read_lfh(&mut self.file, lfh_start)
  }

  pub fn read_lfh_raw_data(&mut self, cdfh: &CentralDirectoryFileHeader) -> io::Result<Vec<u8>> {
    read_lfh_raw_data(&mut self.file, cdfh)
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

  pub fn print_info(&mut self) -> io::Result<()> {
    {
      let cdfh_vec: Vec<CentralDirectoryFileHeader> = try!(self.cd_iter()).collect();  
      for cdfh in cdfh_vec {
        println!("{}", cdfh);
  
        let lfh = try!(self.read_lfh(cdfh.local_file_header_start));
        println!("{}", lfh);
      }
    }
    {
      let eocd = try!(self.read_eocd());
      println!("{}", eocd);
    }
    Ok(())
  }

  pub fn unzip(&mut self) -> io::Result<()> {
    let cdfh_entries: Vec<CentralDirectoryFileHeader> = try!(self.cd_iter()).collect();

    for cdfh in cdfh_entries {
      println!("{}", cdfh);

      let mut dir_builder = DirBuilder::new();
      dir_builder.recursive(true);

      let mut open_opts = OpenOptions::new();
      open_opts.create(true).write(true);

      if cdfh.is_directory() {
        println!("create dir {}", cdfh.file_name);

        let dir_path: &Path = Path::new(&cdfh.file_name);
        try!(dir_builder.create(dir_path));
      }
      else if cdfh.is_file() {
        println!("create file {}", cdfh.file_name);

        let file_path: &Path = Path::new(&cdfh.file_name);
        let mut file = try!(open_opts.open(file_path));

        if cdfh.uncompressed_size != 0 {
          println!("get raw data");
          let raw_data: Vec<u8> = try!(self.read_lfh_raw_data(&cdfh));
          println!("{:?}", raw_data);

          println!("get compression method");
          let compression_method = CompressionMethod::from_code(cdfh.compression_method).unwrap();
          println!("decompress data");
          let uncompressed: Vec<u8> = try!(decompress_file_data(raw_data, compression_method));
          println!("write uncompressed data");
          try!(file.write_all(&uncompressed[..]))
        }
      }
    }

    Ok(())
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
      sig: Signature::CentralDirectoryFileHeader.sig_byte(),
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

  pub fn is_directory(&self) -> bool {
    !self.file_name.is_empty() && self.file_name.ends_with("/")
  }

  pub fn is_file(&self) -> bool {
    !self.file_name.is_empty() && !self.file_name.ends_with("/")
  }
}

impl fmt::Display for CentralDirectoryFileHeader {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "CentralDirectoryFileHeader {{").and(
    writeln!(f, "  sig: {:08x},", self.sig)).and(
    writeln!(f, "  created_ver: {},", self.created_ver)).and(
    writeln!(f, "  extract_ver: {},", self.extract_ver)).and(
    writeln!(f, "  compression_method: {},", self.compression_method)).and(
    writeln!(f, "  crc_32: {},", self.crc_32)).and(
    writeln!(f, "  compressed_size: {},", self.compressed_size)).and(
    writeln!(f, "  uncompressed_size: {},", self.uncompressed_size)).and(
    writeln!(f, "  file_name_len: {},", self.file_name_len)).and(
    writeln!(f, "  extra_field_len: {},", self.extra_field_len)).and(
    writeln!(f, "  comment_len: {},", self.comment_len)).and(
    writeln!(f, "  file_start_disk_num: {},", self.file_start_disk_num)).and(
    writeln!(f, "  local_file_header_start: {},", self.local_file_header_start)).and(
    writeln!(f, "  file_name: \"{}\",", self.file_name)).and(
    writeln!(f, "  extra_field: \"{}\",", self.extra_field)).and(
    writeln!(f, "  comment: \"{}\",", self.comment)).and(
    writeln!(f, "}}"))
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
      sig: Signature::EndOfCentralDirectory.sig_byte(), 
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

impl fmt::Display for EndOfCentralDirectory {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "EndOfCentralDirectory {{ ").and(
    writeln!(f, "  sig: {:08x},", self.sig)).and(
    writeln!(f, "  this_disk_num: {},", self.this_disk_num)).and(
    writeln!(f, "  cd_start_disk: {},", self.cd_start_disk)).and(
    writeln!(f, "  cd_records_on_this_disk: {},", self.cd_records_on_this_disk)).and(
    writeln!(f, "  total_cd_records: {},", self.total_cd_records)).and(
    writeln!(f, "  cd_size_bytes: {},", self.cd_size_bytes)).and(
    writeln!(f, "  cd_start_offset: {},", self.cd_start_offset)).and(
    writeln!(f, "  comment_len: {}", self.comment_len)).and(
    writeln!(f, "}}"))
  }
}

#[repr(packed)]
#[derive(Debug)]
pub struct LocalFileHeader {
  pub sig: u32,
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
  pub file_name: String,
  pub extra_field: String
}

impl LocalFileHeader {
  pub fn new() -> LocalFileHeader {
    LocalFileHeader {
      sig: Signature::LocalFileHeader.sig_byte(),
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
      file_name: String::new(),
      extra_field: String::new()
    }
  }

  pub fn is_directory(&self) -> bool {
    !self.file_name.is_empty() && self.file_name.ends_with("/")
  }

  pub fn is_file(&self) -> bool {
    !self.file_name.is_empty() && !self.file_name.ends_with("/")
  }
}

impl fmt::Display for LocalFileHeader {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "LocalFileHeader {{ ").and(
    writeln!(f, "  sig: {:08x},", self.sig)).and(
    writeln!(f, "  extract_ver: {},", self.extract_ver)).and(
    writeln!(f, "  general_bit_flag: {},", self.general_bit_flag)).and(
    writeln!(f, "  compression_method: {},", self.compression_method)).and(
    writeln!(f, "  last_modified_time: {},", self.last_modified_time)).and(
    writeln!(f, "  last_modified_date: {},", self.last_modified_date)).and(
    writeln!(f, "  crc_32: {},", self.crc_32)).and(
    writeln!(f, "  compressed_size: {},", self.compressed_size)).and(
    writeln!(f, "  uncompressed_size: {},", self.uncompressed_size)).and(
    writeln!(f, "  file_name_len: {},", self.file_name_len)).and(
    writeln!(f, "  extra_field_len: {},", self.extra_field_len)).and(
    writeln!(f, "  file_name: \"{}\",", self.file_name)).and(
    writeln!(f, "  extra_field: \"{}\"", self.extra_field)).and(
    writeln!(f, "}}"))
  }
}
