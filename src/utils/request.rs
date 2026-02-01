pub struct Http;
 
impl Http {
    pub async fn get(url: &str) -> Option<Vec<u8>> {
         let body = match reqwest::get(url)
        .await {
            Ok(v) => v,
            Err(_) => {return None;}
        };
        match body.bytes().await {
            Ok(v) => Some(v.to_vec()),
            Err(_) =>  None
        }
    }
}