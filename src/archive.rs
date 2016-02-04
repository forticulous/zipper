use std::fs::{DirBuilder, File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::path::Path;

use cdfh::CentralDirectoryFileHeader;
use central_directory_iter::CentralDirectoryIter;
use eocd::EndOfCentralDirectory;
use functions::{decompress_file_data, read_lfh, read_lfh_raw_data, read_eocd};
use lfh::LocalFileHeader;

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
    try!(self.file.seek(SeekFrom::Start(cdfh_start as u64)));

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
      let mut dir_builder = DirBuilder::new();
      dir_builder.recursive(true);

      let mut open_opts = OpenOptions::new();
      open_opts.create(true).write(true);

      if cdfh.is_directory() {
        let dir_path: &Path = cdfh.as_path();
        try!(dir_builder.create(dir_path));
      }
      else if cdfh.is_file() {
        if cdfh.compressed_size != 0 {
          let raw_data: Vec<u8> = try!(self.read_lfh_raw_data(&cdfh));

          let compression_method = cdfh.as_compression_method().unwrap();
          let decompressed: Vec<u8> = try!(decompress_file_data(raw_data, compression_method));
          
          let file_path: &Path = cdfh.as_path();
          let mut file = try!(open_opts.open(file_path));

          try!(file.write_all(&decompressed[..]))
        }
      }
    }

    Ok(())
  }
}
