// src/commands/weather.rs
use crate::models::weather_models::WeatherResponse;
use crate::services::weather_services::{WeatherService, WeatherError};

pub async fn handle(args: &[&str], _api_key: &str) {
    if args.is_empty() {
        println!("Usage: weather <city> [country]");
        return;
    }

    let city = args[0];
    let country = args.get(1).map(|&s| s);

    let service = WeatherService::new();

    match service.fetch(city, country).await {
        Ok(weather) => print_weather(&weather),
        Err(WeatherError::LocationNotFound(c)) => println!("City '{}' not found. Try a different spelling.", c),
        Err(WeatherError::ApiError(msg)) => println!("API error: {}", msg),
        Err(WeatherError::RequestFailed(e)) => println!("Network error: {}", e),
    }
}

fn print_weather(weather: &WeatherResponse) {
    println!("Weather for {}, {}", weather.name, weather.country);
    println!("  Temperature: {:.1}°C", weather.temp);
    println!("  Condition: {}", weather.description);
    println!("  Wind: {:.1} km/h", weather.wind_speed);
}