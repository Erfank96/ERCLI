// src/help_command.rs
pub fn print_help() {
    println!(
        r#"Available commands:

  help                – show this help
  exit                – quit the shell

  time now            – print current local time
  time utc            – print current UTC time

  crypto btc          – current Bitcoin price (USD)
  crypto eth          – current Ethereum price (USD)
"#
    );
}