use chrono::Local;
use serde::Deserialize;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

trait Pricing {
    fn fetch_price(&mut self) -> Result<(), String>;
    fn save_to_file(&self) -> Result<(), String>;
}

#[derive(Deserialize, Debug)]
struct Bitcoin {
    price_usd: f64,
}

impl Bitcoin {
    fn new() -> Self {
        Bitcoin { price_usd: 0.0 }
    }
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response = ureq::get(url)
            .set("User-Agent", "financial-data-fetcher/1.0")
            .call()
            .map_err(|e| e.to_string())?;
        let reader = response.into_reader();
        let json: serde_json::Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
        self.price_usd = json["bitcoin"]["usd"]
            .as_f64()
            .ok_or("Failed to parse Bitcoin price")?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), String> {
        let timestamp = Local::now();
        let entry = format!("{}, ${:.2}\n", timestamp.to_rfc3339(), self.price_usd);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("bitcoin_prices.txt")
            .map_err(|e| e.to_string())?;
        file.write_all(entry.as_bytes()).map_err(|e| e.to_string())
    }
}

#[derive(Deserialize, Debug)]
struct Ethereum {
    price_usd: f64,
}

impl Ethereum {
    fn new() -> Self {
        Ethereum { price_usd: 0.0 }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url)
            .set("User-Agent", "financial-data-fetcher/1.0")
            .call()
            .map_err(|e| e.to_string())?;
        let reader = response.into_reader();
        let json: serde_json::Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
        self.price_usd = json["ethereum"]["usd"]
            .as_f64()
            .ok_or("Failed to parse Ethereum price")?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), String> {
        let timestamp = Local::now();
        let entry = format!("{}, ${:.2}\n", timestamp.to_rfc3339(), self.price_usd);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ethereum_prices.txt")
            .map_err(|e| e.to_string())?;
        file.write_all(entry.as_bytes()).map_err(|e| e.to_string())
    }
}

#[derive(Deserialize, Debug)]
struct SP500 {
    price: f64,
}

impl SP500 {
    fn new() -> Self {
        SP500 { price: 0.0 }
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response = ureq::get(url)
            .set("User-Agent", "financial-data-fetcher/1.0")
            .call()
            .map_err(|e| e.to_string())?;
        let reader = response.into_reader();
        let json: serde_json::Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
        self.price = json["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or("Failed to parse S&P 500 price")?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), String> {
        let timestamp = Local::now();
        let entry = format!("{}, ${:.2}\n", timestamp.to_rfc3339(), self.price);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("sp500_prices.txt")
            .map_err(|e| e.to_string())?;
        file.write_all(entry.as_bytes()).map_err(|e| e.to_string())
    }
}

fn main() {
    let mut bitcoin = Bitcoin::new();
    let mut ethereum = Ethereum::new();
    let mut sp500 = SP500::new();

    loop {
        println!("Fetching data...");

        let mut assets: Vec<&mut dyn Pricing> = vec![&mut bitcoin, &mut ethereum, &mut sp500];

        for asset in assets.iter_mut() {
            if let Err(e) = asset.fetch_price() {
                eprintln!("Error fetching price: {}", e);
            }
            if let Err(e) = asset.save_to_file() {
                eprintln!("Error saving to file: {}", e);
            }
        }

        println!("Sleeping for 10 seconds...\n");
        thread::sleep(Duration::from_secs(10)); // Increased to avoid rate limits
    }
}
