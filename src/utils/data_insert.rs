use chrono::NaiveDate;
use regex::Regex;
use sqlx::MySqlPool;

use crate::utils::app_error::AppError;

#[derive(Debug)]
pub struct StockRow {
    pub code: String,
    pub name: String,
    pub unit: i32,

    pub am_open: Option<f64>,
    pub am_high: Option<f64>,
    pub am_low: Option<f64>,
    pub am_close: Option<f64>,

    pub pm_open: Option<f64>,
    pub pm_high: Option<f64>,
    pub pm_low: Option<f64>,
    pub pm_close: Option<f64>,

    pub last_quote: Option<f64>,
    pub change_amount: Option<f64>,
    pub vwap: Option<f64>,
    pub volume_thousand: Option<f64>,
    pub turnover_thousand_yen: Option<f64>,
}

async fn get_or_create_trading_day(pool: &MySqlPool, date: NaiveDate) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO 
        trading_day (trade_date)
        VALUES (?)
        ON DUPLICATE KEY UPDATE id = LAST_INSERT_ID(id)
        "#,
        date
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_id();
    Ok(id)
}

async fn insert_stock(
    pool: &MySqlPool,
    trading_day_id: u64,
    row: StockRow,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO stock_daily_price (
            trading_day_id,
            code,
            name,
            unit,
            am_open, am_high, am_low, am_close,
            pm_open, pm_high, pm_low, pm_close,
            last_quote, change_amount, vwap,
            volume_thousand, turnover_thousand_yen
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            am_open = VALUES(am_open),
            am_high = VALUES(am_high),
            am_low = VALUES(am_low),
            am_close = VALUES(am_close),
            pm_open = VALUES(pm_open),
            pm_high = VALUES(pm_high),
            pm_low = VALUES(pm_low),
            pm_close = VALUES(pm_close),
            last_quote = VALUES(last_quote),
            change_amount = VALUES(change_amount),
            vwap = VALUES(vwap),
            volume_thousand = VALUES(volume_thousand),
            turnover_thousand_yen = VALUES(turnover_thousand_yen)
        "#,
        trading_day_id,
        row.code,
        row.name,
        row.unit,
        row.am_open,
        row.am_high,
        row.am_low,
        row.am_close,
        row.pm_open,
        row.pm_high,
        row.pm_low,
        row.pm_close,
        row.last_quote,
        row.change_amount,
        row.vwap,
        row.volume_thousand,
        row.turnover_thousand_yen
    )
    .execute(pool)
    .await?;

    Ok(())
}

fn parse_trade_date(line: &str) -> Option<NaiveDate> {
    let re = Regex::new(r"(\d{4})(\d{1,2})(\d{1,2})").unwrap();
    let caps = re.captures(line)?;

    let year: i32 = caps[1].parse().ok()?;
    let month: u32 = caps[2].parse().ok()?;
    let day: u32 = caps[3].parse().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
}

fn parse_optional_f64(s: &str) -> Option<f64> {
    let s = s.trim();

    if s.is_empty() || s == "－" {
        return None;
    }

    s.parse::<f64>().ok()
}

pub async fn insert_data(pool: &MySqlPool, data: &str, date: &str) -> Result<(), AppError> {
    let trade_date = parse_trade_date(date)
        .ok_or_else(|| AppError::new(&format!("failed to parse trade date"), file!(), line!()))?;
    let trading_day_id = get_or_create_trading_day(pool, trade_date).await?;

    for line in data.lines() {
        let line = line.trim();

        if line.starts_with("\"!") || line.is_empty() {
            continue;
        }

        // Remove trailing comma if exists
        let clean = line.trim_end_matches(',');

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(clean.as_bytes());

        for result in rdr.records() {
            let record = result?;

            // Skip non-stock rows
            if record.len() < 17 {
                continue;
            }

            let row = StockRow {
                code: record[0].to_string(),
                name: record[3].to_string(),
                unit: record[2].parse().unwrap_or(0),

                am_open: parse_optional_f64(&record[4]),
                am_high: parse_optional_f64(&record[5]),
                am_low: parse_optional_f64(&record[6]),
                am_close: parse_optional_f64(&record[7]),

                pm_open: parse_optional_f64(&record[8]),
                pm_high: parse_optional_f64(&record[9]),
                pm_low: parse_optional_f64(&record[10]),
                pm_close: parse_optional_f64(&record[11]),

                last_quote: parse_optional_f64(&record[12]),
                change_amount: parse_optional_f64(&record[13]),
                vwap: parse_optional_f64(&record[14]),
                volume_thousand: parse_optional_f64(&record[15]),
                turnover_thousand_yen: parse_optional_f64(&record[16]),
            };

            insert_stock(pool, trading_day_id, row).await?;
        }
    }

    Ok(())
}
