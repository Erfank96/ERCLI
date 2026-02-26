// src/models/weather.rs
pub struct WeatherResponse {
    pub name: String,
    pub country: String,
    pub temp: f64,
    pub description: String,
    pub wind_speed: f64,
}