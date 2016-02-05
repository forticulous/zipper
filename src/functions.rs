use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, SeekFrom, Error, ErrorKind};
use std::mem;
use std::slice;

use cdfh::CentralDirectoryFileHeader;
use enums::ArchiveStructure;
use eocd::EndOfCentralDirectory;
use lfh::LocalFileHeader;

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
            slice::from_raw_parts_mut(&mut cdfh as *mut _ as *mut u8, ArchiveStructure::CentralDirectoryFileHeader.constant_size()) 
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
        slice::from_raw_parts_mut(&mut eocd as *mut _ as *mut u8, ArchiveStructure::EndOfCentralDirectory.constant_size())
    };
    try!(file.read_exact(slice));

    Ok(eocd)
}

pub fn read_lfh(file: &mut File, lfh_start: u32) -> io::Result<LocalFileHeader> {
    let mut lfh = LocalFileHeader::new();

    try!(file.seek(SeekFrom::Start(lfh_start as u64)));

    {
        let slice: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(&mut lfh as *mut _ as *mut u8, ArchiveStructure::LocalFileHeader.constant_size())
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
    let lfh = try!(read_lfh(file, cdfh.local_file_header_start));
    let data_start = cdfh.local_file_header_start as usize +
        ArchiveStructure::LocalFileHeader.constant_size() +
        lfh.file_name_len as usize +
        lfh.extra_field_len as usize;
    let data_len = cdfh.compressed_size as usize;

    try!(file.seek(SeekFrom::Start(data_start as u64)));

    let mut buf_read = BufReader::with_capacity(data_len, file);
    let bytes: Vec<u8> = try!(buf_read.fill_buf()).to_vec();

    Ok(bytes)
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

    use enums::{Signature, CompressionMethod};

    #[test]
    fn find_sig_eocd() {
        let bytes: &[u8] = &[0, 0, 0x50, 0x4b, 0x05, 0x06, 0, 0];  
        let mut cursor = Cursor::new(bytes);

        let res = find_sig_position(&mut cursor, Signature::EndOfCentralDirectory.sig_byte());
        assert!(res.is_ok());
        assert_eq!(2u64, res.unwrap());
    }

    #[test]
    fn deflate_from_code() {
        let method = CompressionMethod::from_code(8u16).unwrap();
        assert_eq!(CompressionMethod::Deflate, method);
    }

}
