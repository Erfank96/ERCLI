// src/help_command.rs
pub fn print_help() {
    println!(
        r#"
Available commands:

  General:
    help                - Show this help menu
    exit                - Quit the shell

  Time & Date:
    time now            - Print current local time
    time utc            - Print current UTC time
    time list           - Show time for all configured countries
    time <country>      - Show time for a specific country (e.g., 'time iran')

    date solar          - Print current date in Solar Hijri (1404/12/07)
    date full           - Print Solar Hijri with month names (7 Esfand 1404)
    date all            - Show both Gregorian and Solar dates side-by-side

  Crypto:
    crypto              - Show prices for all supported coins (USD)
    crypto <coins...>   - Show specific coins (e.g., 'crypto btc eth')
                        - Supported: btc, eth, bnb, sol, xrp, ada, doge, trx, avax, shib
"#
    );
}