use std::fmt;
use std::path::Path;

use enums::{CompressionMethod, Signature};

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

    pub fn as_path(&self) -> &Path {
        Path::new(&self.file_name)
    }

    pub fn as_compression_method(&self) -> Option<CompressionMethod> {
        CompressionMethod::from_code(self.compression_method)
    }
}

impl fmt::Display for CentralDirectoryFileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "CentralDirectoryFileHeader {{"));
        try!(writeln!(f, "  sig: {:08x},", self.sig));
        try!(writeln!(f, "  created_ver: {},", self.created_ver));
        try!(writeln!(f, "  extract_ver: {},", self.extract_ver));
        try!(writeln!(f, "  compression_method: {},", self.compression_method));
        try!(writeln!(f, "  crc_32: {},", self.crc_32));
        try!(writeln!(f, "  compressed_size: {},", self.compressed_size));
        try!(writeln!(f, "  uncompressed_size: {},", self.uncompressed_size));
        try!(writeln!(f, "  file_name_len: {},", self.file_name_len));
        try!(writeln!(f, "  extra_field_len: {},", self.extra_field_len));
        try!(writeln!(f, "  comment_len: {},", self.comment_len));
        try!(writeln!(f, "  file_start_disk_num: {},", self.file_start_disk_num));
        try!(writeln!(f, "  local_file_header_start: {},", self.local_file_header_start));
        try!(writeln!(f, "  file_name: \"{}\",", self.file_name));
        try!(writeln!(f, "  extra_field: \"{}\",", self.extra_field));
        try!(writeln!(f, "  comment: \"{}\",", self.comment));
        writeln!(f, "}}")
    }
}
