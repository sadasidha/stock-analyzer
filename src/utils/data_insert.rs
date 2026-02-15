
use sqlx::MySqlPool;
use chrono::NaiveDate;

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


async fn get_or_create_trading_day(
    pool: &MySqlPool,
    date: NaiveDate,
) -> Result<u64, sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO trading_day (trade_date)
        VALUES (?)
        ON DUPLICATE KEY UPDATE id = LAST_INSERT_ID(id)
        "#,
        date
    )
    .execute(pool)
    .await?;

    let id: (u64,) = sqlx::query_as(
        "SELECT LAST_INSERT_ID()"
    )
    .fetch_one(pool)
    .await?;

    Ok(id.0)
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
