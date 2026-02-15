CREATE TABLE trading_day (
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    trade_date DATE NOT NULL,
    UNIQUE KEY uk_trade_date (trade_date)
) ENGINE=InnoDB D

CREATE TABLE stock_daily_price (
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    trading_day_id BIGINT UNSIGNED NOT NULL,
    code VARCHAR(10) NOT NULL,
    name VARCHAR(100) NOT NULL,
    unit INT NOT NULL,

    am_open DECIMAL(10,2),
    am_high DECIMAL(10,2),
    am_low DECIMAL(10,2),
    am_close DECIMAL(10,2),

    pm_open DECIMAL(10,2),
    pm_high DECIMAL(10,2),
    pm_low DECIMAL(10,2),
    pm_close DECIMAL(10,2),

    last_quote DECIMAL(10,2) NULL,
    change_amount DECIMAL(10,2) NULL,
    vwap DECIMAL(12,4) NULL,

    volume_thousand DECIMAL(15,3) NULL,
    turnover_thousand_yen DECIMAL(18,3) NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    UNIQUE KEY uk_day_code (trading_day_id, code),
    INDEX idx_code (code),

    CONSTRAINT fk_trading_day
        FOREIGN KEY (trading_day_id)
        REFERENCES trading_day(id)
        ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
