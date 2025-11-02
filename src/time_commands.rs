
// Handle commands
match input {
    "hello" => println!("Hello there!"),
    "time" => {
        let now = chrono::Local::now();
        println!("Current time: {}", now);
        }
    _ => println!("Unknown command: {}", input),
}