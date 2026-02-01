CREATE TABLE stock_prices (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    stock_code VARCHAR(10) NOT NULL,
    stock_name VARCHAR(100),
    afternoon_close DECIMAL(10,2),
    morning_open DECIMAL(10,2),
    morning_high DECIMAL(10,2),
    morning_low DECIMAL(10,2),
    afternoon_open DECIMAL(10,2),
    afternoon_high DECIMAL(10,2),
    afternoon_low DECIMAL(10,2),
    volume DECIMAL(10,2),
    value DECIMAL(15,2)
) PARTITION BY RANGE (date);