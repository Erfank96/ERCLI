// Handle commands
pub fn timeCommand(args: &[&str]) {

    match args {
        ["now"] => {
            let now = chrono::Local::now();
            println!("Local time: {}", now.format("%Y-%m-%d %H:%M:%S"));
        }
        ["utc"] => {
            let now = chrono::Utc::now();
            println!("UTC time: {}", now.format("%Y-%m-%d %H:%M:%S"));
        }
        _ => println!("Usage: time <now|utc>"),
    }

}