mod time_commands;
mod crypto_commands;
mod help_command;
mod date_commands;

use std::io::{self, Write};

fn main() {
    println!("Welcome to erfan interactive command shell!");
    println!("Type 'help' for a list of commands, 'exit' to quit.\n");
    println!("Type 'exit' to quit.");
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
            "help" => help_command::print_help(),
            // Handle exit
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "time" => time_commands::time_command(&args),
            "crypto" => crypto_commands::handle(&args),
            "date" => date_commands::date_command(&args),
            _ => println!("Unknown command: {}. Type 'help' for usage.", cmd),
        }
    }
}


