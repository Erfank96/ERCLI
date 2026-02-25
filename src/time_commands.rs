use chrono_tz;
// Handle commands
pub fn time_command(args: &[&str]) {

    // Using a tuple: (Display Name, Timezone Object)
    let locations: [(&str, chrono_tz::Tz); 6] = [
        ("Germany", chrono_tz::Europe::Berlin),
        ("Sweden", chrono_tz::Europe::Stockholm),
        ("Iran",    chrono_tz::Asia::Tehran),
        ("UAE", chrono_tz::Asia::Dubai),
        ("USA (NY)", chrono_tz::America::New_York),
        ("USA (LA)", chrono_tz::America::Los_Angeles),
    ];

    match args {
        ["now"] => {
            let now = chrono::Local::now();
            println!("Local time: {}", now.format("%Y-%m-%d %H:%M:%S"));
        }
        ["utc"] => {
            let now = chrono::Utc::now();
            println!("UTC time: {}", now.format("%Y-%m-%d %H:%M:%S"));
        }
        ["list"] => {
            println!("--- Full World Clock ---");
            for (name, tz) in locations {
                print_location(name, tz);
            }
        }
        // This pattern captures any single string (the country name)
        [target] => {
            let found = locations.iter().find(|(name, _)| {
                name.to_lowercase() == target.to_lowercase()
            });

            if let Some((name, tz)) = found {
                print_location(name, *tz);
            } else {
                println!("Unknown location: '{}'. Try 'list' to see options.", target);
            }
        }
        _ => println!("Usage: time <now | utc | list | country_name>"),
    }

    // Helper function to keep the printing logic in one place
    fn print_location(name: &str, tz: chrono_tz::Tz) {
        let time = chrono::Utc::now().with_timezone(&tz);
        println!("{:<12} | {}", name, time.format("%Y-%m-%d %H:%M:%S"));
    }
}