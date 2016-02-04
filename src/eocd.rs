use std::fmt;

use enums::Signature;

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

