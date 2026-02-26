use reqwest::blocking::get;
use serde_json::Value;

/// Mapping: user-friendly symbol → CoinGecko id
const COINS: &[(&str, &str)] = &[
    ("btc", "bitcoin"),
    ("eth", "ethereum"),
    ("bnb", "binancecoin"),
    ("sol", "solana"),
    ("xrp", "ripple"),
    ("ada", "cardano"),
    ("doge", "dogecoin"),
    ("trx", "tron"),
    ("avax", "avalanche-2"),
    ("shib", "shiba-inu"),
];

pub fn handle(args: &[&str]) {
    // Decide which coins to fetch
    let requested: Vec<String> = if args.is_empty() {
        // No args → show ALL
        COINS.iter().map(|&(sym, _)| sym.to_string()).collect()
    } else {
        // Convert args to lowercase symbols
        args.iter().map(|s| s.to_ascii_lowercase()).collect()
    };

    // Validate symbols
    let mut unknown = Vec::new();
    let mut ids_to_fetch = Vec::new();
    for sym in &requested {
        if let Some(&(_, id)) = COINS.iter().find(|&&(s, _)| s == *sym) {
            ids_to_fetch.push(id);
        } else {
            unknown.push(sym.clone());
        }
    }

    if !unknown.is_empty() {
        println!("Warning: Unknown coins: {}", unknown.join(", "));
    }

    if ids_to_fetch.is_empty() {
        println!("No valid coins to show. Try `crypto` or `crypto btc eth`.");
        return;
    }

    // ---------- 2. Build API URL ----------
    let ids = ids_to_fetch.join(",");
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        ids
    );

    // ---------- 3. Fetch & parse ----------
    match get(&url) {
        Ok(resp) => match resp.json::<Value>() {
            Ok(data) => print_table(&requested, &data),
            Err(e) => println!("Failed to parse response: {}", e),
        },
        Err(e) => println!("Network error: {}", e),
    }
}

// ---------------------------------------------------------------

// Add this helper to color text
// ──────────────────────────────────────────────────────────────
//  Helper: colourise text
fn c(text: impl std::fmt::Display, code: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", code, text)
}

// ──────────────────────────────────────────────────────────────
fn print_table(requested_symbols: &[String], data: &Value) {
    const COIN_W: usize = 18;   // width of the coin column
    const PRICE_W: usize = 14;  // width of the price column
    const CHANGE_W: usize = 10; // width of the % change column

    // ── Header ─────────────────────────────────────────────────
    println!();
    println!("{}", c("Live Crypto Prices (USD) – CoinGecko", "36")); // cyan
    println!("┌{}┐", "─".repeat(COIN_W + PRICE_W + CHANGE_W + 6));
    println!(
        "│ {:<COIN_W$} │ {:>PRICE_W$} │ {:>CHANGE_W$} │",
        c("Coin", "1;36"), c("Price (USD)", "1;36"), c("24h %", "1;36")
    );
    println!("├{}┤", "─".repeat(COIN_W + PRICE_W + CHANGE_W + 6));

    // ── Rows ───────────────────────────────────────────────────
    for sym in requested_symbols {
        if let Some(&(_, id)) = COINS.iter().find(|&&(s, _)| s == sym.as_str()) {
            let price = data[id]["usd"].as_f64().unwrap_or(f64::NAN);
            let change = data[id]["usd_24h_change"].as_f64().unwrap_or(0.0);

            // ----- coin name (always left-aligned) -----
            let name = match id {
                "bitcoin" => "Bitcoin (BTC)",
                "ethereum" => "Ethereum (ETH)",
                "binancecoin" => "Binance Coin (BNB)",
                "solana" => "Solana (SOL)",
                "ripple" => "XRP",
                "cardano" => "Cardano (ADA)",
                "dogecoin" => "Dogecoin (DOGE)",
                "tron" => "TRON (TRX)",
                "avalanche-2" => "Avalanche (AVAX)",
                "shiba-inu" => "Shiba Inu (SHIB)",
                _ => id,
            };

            // ----- price (right-aligned, smart decimals) -----
            let price_str = if price.is_nan() {
                c("N/A", "90") // dim gray
            } else if price >= 1_000.0 {
                format!("${:>width$.2}", price as u64, width = PRICE_W - 1)
            } else if price >= 1.0 {
                format!("${:>width$.4}", price, width = PRICE_W - 1)
            } else {
                format!("${:>width$.8}", price, width = PRICE_W - 1)
            };
            let price_col = if change >= 0.0 {
                c(price_str, "32") // green
            } else {
                c(price_str, "31") // red
            };

            // ----- 24 h % change (right-aligned, coloured) -----
            let change_str = if change.is_nan() || change == 0.0 {
                "-".to_string()
            } else if change >= 0.0 {
                c(format!("+{:.2}%", change), "32")
            } else {
                c(format!("{:.2}%", change), "31")
            };
            let change_col = format!("{:>CHANGE_W$}", change_str);

            // ----- final row -----
            println!(
                "│ {:<COIN_W$} │ {} │ {} │",
                name, price_col, change_col
            );
        }
    }

    // ── Footer ─────────────────────────────────────────────────
    println!("└{}┘", "─".repeat(COIN_W + PRICE_W + CHANGE_W + 6));
    println!();
}