use tracing::info;
pub struct Http; 

impl Http {
    pub async fn get(url: &str) -> Result<Vec<u8>, String> {
        info!("URL: {url}");
        let body = match reqwest::get(url).await {
            Ok(v) => v,
            Err(_) => {
                return Err(format!("{}[{}]: api called failed", file!(), line!()));
            }
        };
        if body.status()!=200 {
            info!("got nothing");
            return Err(format!("{}[{}]: api called failed with status {}", file!(), line!(), body.status()));
        }
        match body.bytes().await {
            Ok(v) => Ok(v.to_vec()),
            Err(_) => Err(format!("{}[{}]: extracting body failed", file!(), line!())),
        }
    }
}
