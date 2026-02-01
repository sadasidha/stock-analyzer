use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use glob::glob;
use csv::ReaderBuilder;

fn plot_em() -> Result<(), Box<dyn Error>> {
    let csv_folder = "/mnt/c/Users/johny/stock/data/*.csv";
    let output_png = "/mnt/c/Users/johny/stock/stock_plot.png";

    // Store data as Vec<(date, stock_name, price)>
    let mut all_data: Vec<(chrono::NaiveDate, String, f64)> = Vec::new();

    for entry in glob(csv_folder)? {
        let path = entry?;
        let file = File::open(&path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(file);

        let mut current_date: Option<chrono::NaiveDate> = None;

        for result in rdr.records() {
            let record = result?;
            if record.is_empty() {
                continue;
            }

            let first_cell = record.get(0).unwrap_or("").trim_matches('"');

            // Detect date line
            if let Some(caps) = regex::Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日")?.captures(first_cell) {
                let year: i32 = caps[1].parse()?;
                let month: u32 = caps[2].parse()?;
                let day: u32 = caps[3].parse()?;
                current_date = Some(chrono::NaiveDate::from_ymd_opt(year, month, day)).unwrap();
                continue;
            }

            // Skip metadata
            if first_cell.starts_with('!') {
                continue;
            }

            if current_date.is_none() {
                continue;
            }

            // Parse data row
            let stock_name = record.get(3).unwrap_or("").trim_matches('"').to_string();
            let afternoon_close = record.get(11).unwrap_or("－");
            if afternoon_close == "－" {
                continue;
            }
            let price: f64 = afternoon_close.parse()?;

            all_data.push((current_date.unwrap(), stock_name, price));
        }
    }

    if all_data.is_empty() {
        println!("No data found.");
        return Ok(());
    }

    // Organize data per stock
    use std::collections::HashMap;
    let mut stock_map: HashMap<String, Vec<(chrono::NaiveDate, f64)>> = HashMap::new();
    for (date, stock, price) in all_data {
        stock_map.entry(stock).or_default().push((date, price));
    }

    // Setup plot
    let root = BitMapBackend::new(output_png, (1600, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let dates: Vec<chrono::NaiveDate> = stock_map.values().flat_map(|v| v.iter().map(|(d, _)| *d)).collect();
    let min_date = *dates.iter().min().unwrap();
    let max_date = *dates.iter().max().unwrap();

    let min_price = stock_map.values()
        .flat_map(|v| v.iter().map(|(_, p)| *p))
        .fold(f64::INFINITY, f64::min);
    let max_price = stock_map.values()
        .flat_map(|v| v.iter().map(|(_, p)| *p))
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("株価終値推移", ("IPAGothic", 40))
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_label_formatter(&|d| format!("{}", d.format("%Y-%m-%d")))
        .y_label_formatter(&|v| format!("{:.0}", v))
        .draw()?;

    // Plot each stock
    let colors = &[&RED, &BLUE, &GREEN, &CYAN, &MAGENTA, &BLACK, &YELLOW];
    for (i, (stock, data)) in stock_map.iter().enumerate() {
        let color = colors[i % colors.len()];
        chart.draw_series(LineSeries::new(
            data.clone(),
            color.stroke_width(2),
        ))?.label(stock).legend(move |(x,y)| PathElement::new(vec![(x,y),(x+20,y)], color.stroke_width(2)));
    }

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    println!("Plot saved as {}", output_png);
    Ok(())
}
