
use crate::utils::request::Http;
use crate::utils::unzip::UnzipU8Stream;
pub struct SofthmpUrl;
// https://softhompo.a.la9.jp/Data/stock/thisMonth/stq_20260128.zip

impl SofthmpUrl {
    const SOFTHMP_URL: &'static str = "https://softhompo.a.la9.jp/Data/stock/thisMonth/";

    fn get_path(date: &str) -> String {
        Self::SOFTHMP_URL.to_string() + date
    }

    pub async fn get_stream(date: &str) -> Option<String> {
        let url = Self::get_path(date);
        let zip_bytes = Http::get(&url).await?;
        UnzipU8Stream::unzip_downloaded_stream(&zip_bytes)
    }
}
