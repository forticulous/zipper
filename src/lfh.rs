use std::fmt;

use enums::Signature;

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
        try!(writeln!(f, "LocalFileHeader {{ "));
        try!(writeln!(f, "  sig: {:08x},", self.sig));
        try!(writeln!(f, "  extract_ver: {},", self.extract_ver));
        try!(writeln!(f, "  general_bit_flag: {},", self.general_bit_flag));
        try!(writeln!(f, "  compression_method: {},", self.compression_method));
        try!(writeln!(f, "  last_modified_time: {},", self.last_modified_time));
        try!(writeln!(f, "  last_modified_date: {},", self.last_modified_date));
        try!(writeln!(f, "  crc_32: {},", self.crc_32));
        try!(writeln!(f, "  compressed_size: {},", self.compressed_size));
        try!(writeln!(f, "  uncompressed_size: {},", self.uncompressed_size));
        try!(writeln!(f, "  file_name_len: {},", self.file_name_len));
        try!(writeln!(f, "  extra_field_len: {},", self.extra_field_len));
        try!(writeln!(f, "  file_name: \"{}\",", self.file_name));
        try!(writeln!(f, "  extra_field: \"{}\"", self.extra_field));
        writeln!(f, "}}")
    }
}
