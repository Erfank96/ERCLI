use reqwest::Client;
use crate::models::weather_models::WeatherResponse;

pub struct WeatherService {
    client: Client,
}

#[derive(Debug)]
pub enum WeatherError {
    LocationNotFound(String),
    ApiError(String),
    RequestFailed(reqwest::Error),
}

impl std::fmt::Display for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherError::LocationNotFound(city) => write!(f, "City '{}' not found", city),
            WeatherError::ApiError(msg) => write!(f, "{}", msg),
            WeatherError::RequestFailed(e) => write!(f, "Request failed: {}", e),
        }
    }
}

impl std::error::Error for WeatherError {}

impl From<reqwest::Error> for WeatherError {
    fn from(e: reqwest::Error) -> Self {
        WeatherError::RequestFailed(e)
    }
}

impl WeatherService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch(&self, city: &str, _country: Option<&str>) -> Result<WeatherResponse, WeatherError> {
        // Step 1: Geocode city name to lat/lon
        let geo_url = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
            city
        );

        let geo_resp = self.client.get(&geo_url).send().await?;
        let geo_json: serde_json::Value = geo_resp.json().await?;

        if geo_json.get("results").is_none() {
            return Err(WeatherError::LocationNotFound(city.to_string()));
        }

        let results = geo_json["results"].as_array().unwrap();
        if results.is_empty() {
            return Err(WeatherError::LocationNotFound(city.to_string()));
        }

        let location = &results[0];
        let lat = location["latitude"].as_f64().unwrap();
        let lon = location["longitude"].as_f64().unwrap();
        let name = location["name"].as_str().unwrap_or(city);
        let country_code = location["country_code"].as_str().unwrap_or("");

        // Step 2: Get weather data using lat/lon
        let weather_url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
            lat, lon
        );

        let weather_resp = self.client.get(&weather_url).send().await?;
        let weather_json: serde_json::Value = weather_resp.json().await?;

        let current = &weather_json["current_weather"];

        Ok(WeatherResponse {
            name: name.to_string(),
            country: country_code.to_string(),
            temp: current["temperature"].as_f64().unwrap_or(0.0),
            feels_like: current["temperature"].as_f64().unwrap_or(0.0), // Open-Meteo basic doesn't have feels_like
            description: weather_code_to_description(current["weathercode"].as_i64().unwrap_or(0)),
            humidity: 0, // Not in basic current_weather, would need additional params
            wind_speed: current["windspeed"].as_f64().unwrap_or(0.0),
        })
    }
}

fn weather_code_to_description(code: i64) -> String {
    match code {
        0 => "clear sky",
        1 => "mainly clear",
        2 => "partly cloudy",
        3 => "overcast",
        45 | 48 => "foggy",
        51 | 53 | 55 => "drizzle",
        61 | 63 | 65 => "rain",
        71 | 73 | 75 => "snow",
        80 | 81 | 82 => "rain showers",
        95 | 96 | 99 => "thunderstorm",
        _ => "unknown",
    }.to_string()
}