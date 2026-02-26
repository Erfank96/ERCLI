use chrono;
use parsidate;

pub fn date_command(args: &[&str]){
    let now_gregorian_date = chrono::Local::now().date_naive();
    let solar_date= match parsidate::ParsiDate::from_gregorian(now_gregorian_date) {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Error: Could not convert date to Solar Hijri ({})", e);
            return; // Exit the function early if there is an error
        }
    };
    // 3. Match user arguments
    match args {
        ["solar"] => {
            println!("{:<12} | {}", "Solar Date", solar_date.format("%Y/%m/%d"));
        }
        ["full"] => {
            println!("{:<12} | {}", "Full Date", solar_date.format("%d %B %Y"));
        }
        ["all"] => {
            println!("{:<12} | {}", "Gregorian", now_gregorian_date.format("%Y-%m-%d"));
            println!("{:<12} | {}", "Solar", solar_date.format("%Y/%m/%d"));
        }
        _ => {
            println!("Usage: date <solar | full | all>");
        }
    }
}