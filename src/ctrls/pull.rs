use rocket::Route;

use crate::utils::softhmpo_url::SofthmpUrl;
use tracing::info;


#[get("/pull?<date>")]
pub async fn pull(date: String) -> Result<String, String> {
    let date = format_date(date)?;
    SofthmpUrl::get_stream(&date).await
}

fn format_date(date: String) -> Result<String, String> {
    let mut d = "".to_string();
    let mut count = 0;
    for x in date.as_bytes() {
        if *x>=0x30 && *x<=0x39 {
            d.push(*x as char);
            count +=1;
        }
        if count >=10 {
            break;
        }
    }
    if d.len()!=8 {
        return Err(format!("{}[{}]: failed to parse date", file!(), line!()));
    }
    Ok(d)
}


pub fn mount() -> Vec<Route> {
    routes![pull]
}