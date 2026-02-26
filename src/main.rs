mod commands;
mod models;
mod services;

use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("Welcome to erfan interactive command shell!");
    println!("Type 'help' for a list of commands, 'exit' to quit.\n");
    println!("Type 'exit' to quit.");

    // Load API key from env or config
    let weather_api_key = std::env::var("OPENWEATHER_API_KEY")
        .unwrap_or_else(|_| "demo".to_string());

    loop {
        // Print prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("Error reading input");
            continue;
        }
        let line = line.trim();

        // Empty line → continue
        if line.is_empty() {
            continue;
        }

        // Split into command + args
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap();               // we already know there is at least one token
        let args: Vec<&str> = parts.collect();

        // Dispatch
        match cmd {
            "help" => commands::help_command::print_help().await,
            // Handle exit
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "time" => commands::time_commands::time_command(&args),
            "crypto" => commands::crypto_commands::handle(&args),
            "date" => commands::date_commands::date_command(&args),
            "weather" => commands::weather_commands::handle(&args, &weather_api_key).await,
            //"version" => commands::version_command::run(&args).await,
            _ => println!("Unknown command: {}. Type 'help' for usage.", cmd),
        }
    }
}


