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
