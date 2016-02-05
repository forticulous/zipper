use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use zip_data::ZipData;
use functions::{read_cdfh, read_lfh_raw_data};

pub struct ZipDataIter<'a> {
    pub file: &'a mut File
}

impl<'a> Iterator for ZipDataIter<'a> {
    type Item = ZipData;

    fn next(&mut self) -> Option<ZipData> {
        let result = read_cdfh(self.file);
        if result.is_err() {
            return None;
        }

        let cdfh = result.unwrap();
        let compression_method = cdfh.as_compression_method().unwrap();

        let current = self.file.seek(SeekFrom::Current(0)).unwrap();
        let raw_data = read_lfh_raw_data(self.file, &cdfh).unwrap();
        self.file.seek(SeekFrom::Start(current)).unwrap();

        let zip_data = ZipData::new(cdfh.file_name, compression_method, raw_data);
        Some(zip_data)
    }
}
