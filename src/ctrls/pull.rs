use rocket::Route;

use crate::utils::softhmpo_url::SofthmpUrl;


#[get("/pull?<date>")]
pub async fn pull(date: String) -> String {
    let date = format_date(date);
    if date.is_empty() {
        return date;
    }
    SofthmpUrl::get_stream(&date).await.unwrap_or("".to_string())
}

fn format_date(date: String) -> String {
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
    if d.len()!=10 {
        return "".to_string();
    }
    d
}


pub fn mount() -> Vec<Route> {
    routes![pull]
}