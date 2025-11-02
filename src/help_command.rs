// src/help_command.rs
pub fn print_help() {
    println!(
        r#"Available commands:

  help                – show this help
  exit                – quit the shell

  time now            – print current local time
  time utc            – print current UTC time

r#"  crypto [coins...]   – show crypto prices (USD)
        crypto          → all coins
        crypto btc      → only Bitcoin
        crypto eth sol  → Ethereum + Solana
        Supported: btc, eth, bnb, sol, xrp, ada, doge, trx, avax, shib"#
    );
}