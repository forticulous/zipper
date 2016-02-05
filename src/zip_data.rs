use std::io::{self, Error, ErrorKind};
use std::io::prelude::*;
use std::path::Path;

extern crate flate2;
use self::flate2::write::DeflateDecoder;

use enums::CompressionMethod;

pub struct ZipData {
    pub file_name: String,
    pub compression_method: CompressionMethod,
    raw: Vec<u8>
}

impl ZipData {
    pub fn new(file_name: String, compression_method: CompressionMethod, raw_data: Vec<u8>) -> ZipData {
        ZipData {
            file_name: file_name,
            compression_method: compression_method,
            raw: raw_data
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

    pub fn compressed_size(&self) -> usize {
        self.raw.len()
    }

    pub fn into_decompressed_bytes(self) -> io::Result<Vec<u8>> {
        if CompressionMethod::Store == self.compression_method {
            return Ok(self.raw);
        }
        if CompressionMethod::Deflate != self.compression_method {
            return Err(Error::new(ErrorKind::Other, "Unsupported compression method"));
        }

        let decompressed: Vec<u8> = {
            let vec = Vec::with_capacity(self.raw.len());
            let mut decompressor = DeflateDecoder::new(vec);
            try!(decompressor.write_all(&self.raw[..]));
            try!(decompressor.finish())
        };

        Ok(decompressed)
    }

    pub fn into_compressed_bytes(self) -> Vec<u8> {
        self.raw
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;

    use enums::CompressionMethod;

    extern crate flate2;
    use self::flate2::Compression;
    use self::flate2::write::DeflateEncoder;

    #[test]
    fn deflate_decompress() {
        let bytes: &[u8] = b"herp\x0a";

        let compressed: Vec<u8> = {
            let vec: Vec<u8> = Vec::new();
            let mut encoder = DeflateEncoder::new(vec, Compression::Default);
            encoder.write_all(bytes).unwrap();
            encoder.finish().unwrap()
        };
        let zip_data = ZipData::new(String::from("herp.txt"), CompressionMethod::Deflate, compressed);
        let decompressed = zip_data.into_decompressed_bytes().unwrap();

        assert_eq!(bytes, &decompressed[..]);
    }

}
