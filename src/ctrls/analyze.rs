use crate::utils::app_error::AppError;
use bigdecimal::ToPrimitive;
use linfa::prelude::*;
use linfa_logistic::FittedLogisticRegression;
use linfa_logistic::LogisticRegression;
use ndarray::Array2;
use ndarray::Array1;
use sqlx::types::BigDecimal;

use sqlx::MySqlPool;

pub struct StockRow {
    pub trading_day_id: u64,
    pub am_open: Option<BigDecimal>,
    pub am_high: Option<BigDecimal>,
    pub am_low: Option<BigDecimal>,
    pub am_close: Option<BigDecimal>,
    pub pm_open: Option<BigDecimal>,
    pub pm_high: Option<BigDecimal>,
    pub pm_low: Option<BigDecimal>,
    pub pm_close: Option<BigDecimal>,
    pub last_quote: Option<BigDecimal>,
    pub vwap: Option<BigDecimal>,
    pub volume_thousand: Option<BigDecimal>,
}


pub async fn get_company_data(pool: &MySqlPool, code: &str) -> Result<Vec<StockRow>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT `trading_day_id`, `am_open`, `am_high`, `am_low`, `am_close`,
               `pm_open`, `pm_high`, `pm_low`, `pm_close`,
               `last_quote`, `vwap`, `volume_thousand`
        FROM stock_daily_price
        WHERE code = ?
        ORDER BY trading_day_id ASC
        "#,
        code
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| StockRow {
        trading_day_id: r.trading_day_id,
        am_open: r.am_open,
        am_high: r.am_high,
        am_low: r.am_low,
        am_close: r.am_close,
        pm_open: r.pm_open,
        pm_high: r.pm_high,
        pm_low: r.pm_low,
        pm_close: r.pm_close,
        last_quote: r.last_quote,
        vwap: r.vwap,
        volume_thousand: r.volume_thousand,
    }).collect())
}

pub fn build_features(data: &[StockRow]) -> Vec<Vec<f64>> {
    let mut features = Vec::new();

    for i in 1..data.len() {
        let today = &data[i];
        let yesterday = &data[i - 1];

        // Convert BigDecimal -> f64 safely
        let today_pm_close = today.pm_close.as_ref().unwrap().to_f64().unwrap();
        let yesterday_pm_close = yesterday.pm_close.as_ref().unwrap().to_f64().unwrap();

        let today_am_close = today.am_close.as_ref().unwrap().to_f64().unwrap();
        let today_pm_open = today.pm_open.as_ref().unwrap().to_f64().unwrap();

        let today_last_quote = today.last_quote.as_ref().unwrap().to_f64().unwrap();
        let today_vwap = today.vwap.as_ref().unwrap().to_f64().unwrap();

        let today_volume = today.volume_thousand.as_ref().unwrap().to_f64().unwrap();
        let yesterday_volume = yesterday.volume_thousand.as_ref().unwrap().to_f64().unwrap();

        // Features
        let log_return = (today_pm_close / yesterday_pm_close).ln();
        let am_pm_gap = today_pm_open - today_am_close;
        let vwap_dev = today_last_quote - today_vwap;
        let volume_ratio = if today_volume > 0.0 {
            today_volume / yesterday_volume
        } else {
            1.0
        };

        features.push(vec![log_return, am_pm_gap, vwap_dev, volume_ratio]);
    }

    features
}


pub fn build_target(data: &[StockRow]) -> Vec<u8> {
    let mut target = Vec::new();

    for i in 1..data.len() {
        let next_return = (data[i].pm_close / data[i - 1].pm_close).ln();
        target.push(if next_return > 0.0 { 1 } else { 0 });
    }

    target
}



pub fn train_model(
    features: Vec<Vec<f64>>,
    target: Vec<u8>
) -> Result<FittedLogisticRegression<f64, u8>, linfa::error::Error> {
    let n_samples = features.len();
    let n_features = features[0].len();

    let x: Array2<f64> = Array2::from_shape_vec(
        (n_samples, n_features),
        features.into_iter().flatten().collect()
    ).unwrap();

    let y: Array1<u8> = Array1::from(target);

    LogisticRegression::default()
        .fit(&x, &y, AppError::new("Model training failed", file!(), line!()))
}




pub fn predict_next_day(
    model: &FittedLogisticRegression<f64, u8>,
    features: Vec<f64>
) -> (u8, f64) {
    let x = Array2::from_shape_vec((1, features.len()), features).unwrap();
    let probas = model.predict_proba(&x);
    let prob_up = probas[[0, 1]]; // Probability of class 1 (up)
    let signal = if prob_up > 0.5 { 1 } else { 0 };
    (signal, prob_up)
}

pub async fn store_prediction(
    pool: &MySqlPool,
    trading_day_id: u64,
    code: &str,
    signal: u8,
    prob_up: f64
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO stock_prediction (trading_day_id, code, `signal`, prob_up)
        VALUES (?, ?, ?, ?)
        "#,
        trading_day_id,
        code,
        signal,
        prob_up
    )
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn analyze(pool: &sqlx::MySqlPool) -> Result<(), AppError> {
let codes: Vec<String> = sqlx::query!("SELECT DISTINCT code FROM stock_daily_price")
    .fetch_all( pool)
    .await?
    .into_iter()
    .map(|r| r.code)
    .collect();

    for code in codes {
    let data = get_company_data(&pool, &code).await?;
    if data.len() < 10 { continue; }
    let features = build_features(&data);
    let target = build_target(&data);
    let model = train_model(features.clone(), target);
    let model = train_model(features.clone(), target)?;
    let (signal, prob_up) = predict_next_day(&model, features.last().unwrap().clone());
    store_prediction(&pool, data.last().unwrap().trading_day_id, &code, signal, prob_up).await?;
}