use std::io::{Cursor, Read};

use encoding_rs::SHIFT_JIS;
use zip::ZipArchive;

pub struct UnzipU8Stream;

impl UnzipU8Stream {
    pub fn unzip_downloaded_stream(zip_bytes: &[u8]) -> Option<String> {
        let reader = Cursor::new(zip_bytes);
        let mut archive = match ZipArchive::new(reader) {
            Ok(a) => a,
            Err(_) => {
                return None;
            }
        };

        if archive.len() != 1 {
            return None;
        }

        let mut file = match archive.by_index(0) {
            Ok(r) => r,
            Err(_) => return None,
        };

        if file.is_dir() {
            return None;
        }

        let mut bytes = Vec::new();
        match file.read_to_end(&mut bytes) {
            Ok(_) => {}
            Err(_) => {
                return None;
            }
        };

        // Try UTF-8 first, fallback to Shift_JIS
        let text = match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => {
                let (cow, _, _) = SHIFT_JIS.decode(e.as_bytes());
                cow.into_owned()
            }
        };

        Some(text)
    }
}
