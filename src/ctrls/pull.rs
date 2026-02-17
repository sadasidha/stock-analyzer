
use sqlx::MySqlPool;

use crate::utils::{app_error::AppError, data_insert::insert_data, softhmpo_url::SofthmpUrl};

pub async fn pull(pool: &MySqlPool, date: String) -> Result<String, AppError> {
    let date = format_date(date)?;
    let stream_str = SofthmpUrl::get_stream(&date).await?;
    insert_data(pool, &stream_str, &date).await?;
    Ok(format!(
        "data for {} pulled and inserted successfully",
        date
    ))
}

fn format_date(date: String) -> Result<String, AppError> {
    let mut d = "".to_string();
    let mut count = 0;
    for x in date.as_bytes() {
        if *x >= 0x30 && *x <= 0x39 {
            d.push(*x as char);
            count += 1;
        }
        if count >= 10 {
            break;
        }
    }
    if d.len() != 8 {
        return Err(AppError::new("failed to parse date", file!(), line!()));
    }
    Ok(d)
}
