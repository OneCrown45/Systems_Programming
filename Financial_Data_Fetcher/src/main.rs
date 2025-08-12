use serde::Deserialize;
use std::{env, fs::File, io::Write, thread, time::Duration};

trait Pricing {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>>;
    fn save_to_file(&self, price: f64) -> std::io::Result<()>;
}

struct SP500;
struct Bitcoin;
struct Ethereum;

#[derive(Debug, Deserialize)]
struct YahooFinanceResponse { chart: Chart }
#[derive(Debug, Deserialize)]
struct Chart { result: Vec<ResultData> }
#[derive(Debug, Deserialize)]
struct ResultData { meta: Meta }
#[derive(Debug, Deserialize)]
struct Meta { regularMarketPrice: f64 }

fn fetch_from_yahoo(ticker: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("https://query2.finance.yahoo.com/v8/finance/chart/{}", ticker);
    let body: String = ureq::get(&url).call()?.into_string()?;
    let parsed: YahooFinanceResponse = serde_json::from_str(&body)?;
    Ok(parsed.chart.result[0].meta.regularMarketPrice)
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_from_yahoo("%5EGSPC")
    }
    fn save_to_file(&self, price: f64) -> std::io::Result<()> {
        let mut file = File::create("sp500.txt")?;
        writeln!(file, "{}", price)?;
        Ok(())
    }
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_from_yahoo("BTC-USD")
    }
    fn save_to_file(&self, price: f64) -> std::io::Result<()> {
        let mut file = File::create("bitcoin.txt")?;
        writeln!(file, "{}", price)?;
        Ok(())
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_from_yahoo("ETH-USD")
    }
    fn save_to_file(&self, price: f64) -> std::io::Result<()> {
        let mut file = File::create("ethereum.txt")?;
        writeln!(file, "{}", price)?;
        Ok(())
    }
}

fn main() {
    // Default interval in seconds
    let mut interval_secs = 10;

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 && args[1] == "--interval" {
        if let Ok(val) = args[2].parse::<u64>() {
            interval_secs = val;
        } else {
            eprintln!("Invalid interval '{}', using default {} seconds", args[2], interval_secs);
        }
    }

    println!("Fetching every {} seconds...", interval_secs);

    let assets: Vec<(&str, Box<dyn Pricing>)> = vec![
        ("S&P 500", Box::new(SP500)),
        ("Bitcoin", Box::new(Bitcoin)),
        ("Ethereum", Box::new(Ethereum)),
    ];

    loop {
        for (name, asset) in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("{} Price: {}", name, price);
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Failed to save {}: {}", name, e);
                    }
                }
                Err(e) => eprintln!("Failed to fetch {}: {}", name, e),
            }
        }
        thread::sleep(Duration::from_secs(interval_secs));
    }
}
