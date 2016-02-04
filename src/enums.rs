#[derive(Debug)]
pub enum ArchiveStructure {
    CentralDirectoryFileHeader,
    EndOfCentralDirectory,
    LocalFileHeader
}

impl ArchiveStructure {
    pub fn constant_size(&self) -> usize {
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
