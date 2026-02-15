
use crate::utils::request::Http;
use crate::utils::unzip::UnzipU8Stream;
pub struct SofthmpUrl;
// https://softhompo.a.la9.jp/Data/stock/thisMonth/stq_20260128.zip

impl SofthmpUrl {

    fn get_path(date: &str) -> String {
        format!("https://softhompo.a.la9.jp/Data/stock/thisMonth/stq_{date}.zip")
    }

    pub async fn get_stream(date: &str) -> Result<String, String> {
        let url = Self::get_path(date);
        let zip_bytes = Http::get(&url).await?;
        UnzipU8Stream::unzip_downloaded_stream(&zip_bytes)
    }
}
