use std::io::{Cursor, Read};

use encoding_rs::SHIFT_JIS;
use zip::ZipArchive;

use crate::utils::request::Http;


pub struct SofthmpUrl {
    //https://softhompo.a.la9.jp/Data/stock/thisMonth/stq_20260128.zip
}

impl SofthmpUrl {
    const SOFTHMP_URL: &'static str = "https://softhompo.a.la9.jp/Data/stock/thisMonth/";

    fn get_path(date: &str) -> String {
        Self::SOFTHMP_URL.to_string() + date
    }

    pub async fn get_stream(date: &str) -> Option<String> {
        let url = Self::get_path(date);
        let r = match Http::get(&url).await {
            Some(r) => r,
            None => {
                return None;
            }
        };
        

        None
    }

    pub fn unzip_single_file_to_text(zip_bytes: &[u8]) -> Option<String> {
    let reader = Cursor::new(zip_bytes);
    let mut archive =  match ZipArchive::new(reader) {
        Ok(a) => a,
        Err(_) => {return None;}
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
        Ok(_)=> {},
        Err(_)=> {return None;},
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
//