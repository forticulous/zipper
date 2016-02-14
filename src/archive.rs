use std::fs::{DirBuilder, File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::path::{Path, PathBuf};

use cdfh::CentralDirectoryFileHeader;
use eocd::EndOfCentralDirectory;
use functions::{read_lfh, read_lfh_raw_data, read_eocd, read_cdfh};
use lfh::LocalFileHeader;
use zip_data_iter::ZipDataIter;

pub struct Archive {
    file: File
}

impl Archive {
    pub fn new<T: AsRef<Path>>(path: T) -> io::Result<Archive> {
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

    fn seek_to_cd_start(&mut self) -> io::Result<()> {
        let eocd = try!(self.read_eocd());
        let cdfh_start: u32 = eocd.cd_start_offset;
        try!(self.file.seek(SeekFrom::Start(cdfh_start as u64)));
        Ok(())
    }
     
    pub fn zip_data_iter(&mut self) -> io::Result<ZipDataIter> {
        try!(self.seek_to_cd_start());

        let iter = ZipDataIter {
            file: &mut self.file
        };
        Ok(iter)
    }

    pub fn print_info(&mut self) -> io::Result<()> {
        try!(self.seek_to_cd_start());
        {
            loop {
                let result = read_cdfh(&mut self.file);
                if result.is_err() {
                    break;
                }
                let cdfh = result.unwrap();
                println!("{}", cdfh);

                let current = try!(self.file.seek(SeekFrom::Current(0)));
                let lfh = try!(self.read_lfh(cdfh.local_file_header_start));
                try!(self.file.seek(SeekFrom::Start(current)));

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
        let zip_data_iter = try!(self.zip_data_iter());

        for zip_data in zip_data_iter {
            let mut dir_builder = DirBuilder::new();
            dir_builder.recursive(true);

            let mut open_opts = OpenOptions::new();
            open_opts.create(true).write(true);

            if zip_data.is_directory() {
                let dir_path: &Path = zip_data.as_path();
                try!(dir_builder.create(dir_path));
            }
            else if zip_data.is_file() {
                if zip_data.compressed_size() != 0 {
                    let file_path: PathBuf = zip_data.as_path().to_path_buf();
                    let decompressed: Vec<u8> = try!(zip_data.into_decompressed_bytes());

                    let mut file = try!(open_opts.open(file_path));

                    try!(file.write_all(&decompressed))
                }
            }
        }

        Ok(())
    }
}
