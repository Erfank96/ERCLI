// src/models/weather.rs
pub struct WeatherResponse {
    pub name: String,
    pub country: String,
    pub temp: f64,
    pub feels_like: f64,
    pub description: String,
    pub humidity: u8,
    pub wind_speed: f64,
}