use reqwest::blocking::get;
use serde_json::Value;

pub fn handle(_args: &[&str]) {
    // All popular coins in one call (free & fast)
    let coin_ids = "bitcoin,ethereum,binancecoin,solana,ripple,cardano,dogecoin,tron,avalanche-2,shiba-inu";
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true",
        coin_ids
    );

    match get(&url) {
        Ok(resp) => match resp.json::<Value>() {
            Ok(data) => {
                println!("\nLive Crypto Prices (USD) - CoinGecko:");
                println!("| Rank | Coin          | Price      | 24h Change |");
                println!("|------|---------------|------------|------------|");

                let mut coins = vec![
                    ("bitcoin", "BTC"),
                    ("ethereum", "ETH"),
                    ("binancecoin", "BNB"),
                    ("solana", "SOL"),
                    ("ripple", "XRP"),
                    ("cardano", "ADA"),
                    ("dogecoin", "DOGE"),
                    ("tron", "TRX"),
                    ("avalanche-2", "AVAX"),
                    ("shiba-inu", "SHIB"),
                ];

                for (id, symbol) in coins {
                    if let Some(price_obj) = data.get(id) {
                        let price = price_obj["usd"].as_f64().unwrap_or(0.0);
                        let change = price_obj["usd_24h_change"].as_f64().unwrap_or(0.0);
                        let change_str = if change >= 0.0 {
                            format!("+{:.2}%", change)
                        } else {
                            format!("{:.2}%", change)
                        };

                        println!("|      | {} ({}) | ${:>10.4} | {:>9} |",
                                 match id {
                                     "bitcoin" => "Bitcoin",
                                     "ethereum" => "Ethereum",
                                     "binancecoin" => "Binance Coin",
                                     "solana" => "Solana",
                                     "ripple" => "XRP",
                                     "cardano" => "Cardano",
                                     "dogecoin" => "Dogecoin",
                                     "tron" => "TRX",
                                     "avalanche-2" => "Avalanche",
                                     "shiba-inu" => "Shiba Inu",
                                     _ => id,
                                 },
                                 symbol, price, change_str);
                    }
                }
                println!();
            }
            Err(e) => println!("Failed to parse: {}", e),
        },
        Err(e) => println!("Network error: {}", e),
    }
}