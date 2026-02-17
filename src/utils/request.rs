use tracing::info;

use crate::utils::app_error::AppError;
pub struct Http;

impl Http {
    pub async fn get(url: &str) -> Result<Vec<u8>, AppError> {
        info!("URL: {url}");
        let body = match reqwest::get(url).await {
            Ok(v) => v,
            Err(_) => {
                return Err(AppError::new("api called failed", file!(), line!()));
            }
        };
        if body.status() != 200 {
            info!("got nothing");
            return Err(AppError::new(
                &format!("api called failed with status {}", body.status()),
                file!(),
                line!(),
            ));
        }
        match body.bytes().await {
            Ok(v) => Ok(v.to_vec()),
            Err(_) => Err(AppError::new(
                &format!("extracting body failed"),
                file!(),
                line!(),
            )),
        }
    }
}
