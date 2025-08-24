use std::io::{self, Write};

fn main() {
    println!("Welcome to erfan interactive command shell! Type 'exit' to quit.");
    println!("Type 'exit' to quit.");
    loop {
        // Print prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Handle exit
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Handle commands
        match input {
            "hello" => println!("Hello there!"),
            "time" => {
                let now = chrono::Local::now();
                println!("Current time: {}", now);
            }
            _ => println!("Unknown command: {}", input),
        }
    }
}

