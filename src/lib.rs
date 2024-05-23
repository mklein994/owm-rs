//! Strongly typed models for OpenWeatherMap's "One Call" API:
//! <https://openweathermap.org/api/one-call-3>

use chrono::prelude::*;
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct OwmError {
    #[serde(rename = "cod")]
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    String(String),
    Number(i32),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Number(n) => n.fmt(f),
        }
    }
}

impl fmt::Display for OwmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OWM error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for OwmError {}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub current: Option<Current>,
    pub minutely: Option<Vec<Minutely>>,
    pub hourly: Option<Vec<Hourly>>,
    pub daily: Option<Vec<Daily>>,
    pub alerts: Option<Vec<Alert>>,
}

/// Current weather data API response
#[derive(Debug, Deserialize)]
pub struct Current {
    /// Current time, unix, UTC
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,

    /// Sunrise time, unix, UTC
    #[serde(with = "ts_seconds")]
    pub sunrise: DateTime<Utc>,

    /// Sunset time, unix, UTC
    #[serde(with = "ts_seconds")]
    pub sunset: DateTime<Utc>,

    /// Temperature. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp: f64,

    /// Temperature. This temperature parameter accounts for the human perception of weather.
    ///
    /// Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub feels_like: f64,

    /// Atmospheric pressure on the sea level, hPa
    pub pressure: u16,

    /// Humidity, %
    pub humidity: u8,

    /// Atmospheric temperature (varying according to pressure and humidity) below which water droplets begin to condense and dew can form. Units – default: kelvin, metric: Celsius, imperial: Fahrenheit.
    pub dew_point: f64,

    /// Cloudiness, %
    pub clouds: u8,

    /// Current UV index
    pub uvi: f64,

    /// Average visibility, metres. The maximum value of the visibility is 10km
    pub visibility: Option<u16>,

    /// Wind speed. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour.
    pub wind_speed: f64,

    /// (where available) Wind gust. Units – default: metre/sec, metric: metre/sec, imperial: miles/hour. [How to change units used](https://openweathermap.org/api/one-call-api#data)
    pub wind_gust: Option<f64>,

    /// Wind direction, degrees (meteorological)
    pub wind_deg: u16,

    /// (where available) Rain volume for last hour, mm
    pub rain: Option<Precipitation>,

    /// (where available) Snow volume for last hour, mm
    pub snow: Option<Precipitation>,

    pub weather: Vec<WeatherElement>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherElement {
    /// Weather condition id
    pub id: i64,

    /// Group of weather parameters (Rain, Snow, Extreme, etc.)
    pub main: Main,

    /// Weather condition within the group.
    pub description: String,

    /// Weather icon id.
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub enum Main {
    Thunderstorm,
    Drizzle,
    Rain,
    Snow,
    Mist,
    Smoke,
    Haze,
    Dust,
    Fog,
    Sand,
    Ash,
    Squall,
    Tornado,
    Clear,
    Clouds,
}

/// Minute forecast weather data API response
#[derive(Debug, Deserialize)]
pub struct Minutely {
    /// Time of the forecasted data, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,

    /// Precipitation volume, mm
    pub precipitation: f64,
}

/// Hourly forecast weather data API response
#[derive(Debug, Deserialize)]
pub struct Hourly {
    /// Time of the forecasted data, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,

    /// Temperature. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit. [How
    /// to change units used](https://openweathermap.org/api/one-call-api#data)
    pub temp: f64,

    /// Temperature. This temperature parameter accounts for the human perception of weather.
    pub feels_like: f64,

    /// Atmospheric pressure on the sea level. hPa
    pub pressure: u16,

    /// Humidity, %
    pub humidity: u8,

    /// Atmospheric temperature (varying according to pressure and humidity) below which water droplets begin to condense and dew can form. Units – default: kelvin, metric: Celsius, imperial: Fahrenheit.
    pub dew_point: f64,

    /// UVI index
    pub uvi: f64,

    /// Cloudiness, %
    pub clouds: u8,

    /// Average visibility, metres. The maximum value of the visibility is 10km
    pub visibility: Option<u16>,

    /// Wind speed. Units – default: metre/sec, metric: metre/sec, imperial: miles/hour. [How to change units used](https://openweathermap.org/api/one-call-api#data)
    pub wind_speed: f64,

    /// (where available) Wind gust. Units – default: metre/sec, metric: metre/sec, imperial: miles/hour. [How to change units used](https://openweathermap.org/api/one-call-api#data)
    pub wind_gust: Option<f64>,

    /// Wind direction, degrees (meteorological)
    pub wind_deg: u16,

    /// Probability of precipitation. The values of the parameter vary between 0 and 1, where 0 is equal to 0%, 1 is equal to 100%
    pub pop: f64,

    /// (where available) Rain volume for last hour, mm
    pub rain: Option<Precipitation>,

    /// (where available) Snow volume for last hour, mm
    pub snow: Option<Precipitation>,

    /// Hourly weather element
    pub weather: Vec<WeatherElement>,
}

#[derive(Debug, Deserialize)]
pub struct Precipitation {
    #[serde(rename = "1h")]
    pub one_hour: f64,
}

/// Daily forecast weather data API response
#[derive(Debug, Deserialize)]
pub struct Daily {
    /// Time of the forecasted data, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,

    /// Sunrise time, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub sunrise: DateTime<Utc>,

    /// Sunset time, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub sunset: DateTime<Utc>,

    /// The time of when the moon sets for the day, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub moonrise: DateTime<Utc>,

    /// The time of when the moon sets for the day, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub moonset: DateTime<Utc>,

    /// Moon phase. `0` and `1` are 'new moon', `0.25` is 'first quarter moon', `0.5` is 'full moon' and `0.75` is 'last quarter moon'. The periods in between are called 'waxing crescent', 'waxing gibous', 'waning gibous', and 'waning crescent', respectively.
    pub moon_phase: f64,

    /// Units – default: kelvin, metric: Celsius, imperial: Fahrenheit. [How to change units used](https://openweathermap.org/api/one-call-api#data)
    pub temp: DailyTemperature,

    /// This accounts for the human perception of weather. Units – default: kelvin, metric: Celsius, imperial: Fahrenheit. [How to change units used](https://openweathermap.org/api/one-call-api#data)
    pub feels_like: DailyFeelsLikeTemperature,

    /// Atmospheric pressure on the sea level. hPa
    pub pressure: u16,

    /// Humidity, %
    pub humidity: u8,

    /// Atmospheric temperature (varying according to pressure and humidity) below which water droplets begin to condense and dew can form. Units – default: kelvin, metric: Celsius, imperial: Fahrenheit.
    pub dew_point: f64,

    /// Wind speed. Units – default: metre/sec, metric: metre/sec, imperial: miles/hour. [How to change units used](https://openweathermap.org/api/one-call-api#data)
    pub wind_speed: f64,

    /// (where available) Wind gust. Units – default: metre/sec, metric: metre/sec, imperial: miles/hour. [How to change units used](https://openweathermap.org/api/one-call-api#data)
    pub wind_gust: Option<f64>,

    /// Wind direction, degrees (meteorological)
    pub wind_deg: u16,

    /// Cloudiness, %
    pub clouds: u8,

    /// The maximum value of UV index for the day
    pub uvi: f64,

    /// Probability of precipitation. The values of the parameter vary between 0 and 1, where 0 is equal to 0%, 1 is equal to 100%
    pub pop: f64,

    /// (where available) Precipitation volume, mm
    pub rain: Option<f64>,

    /// (where available) Snow volume, mm
    pub snow: Option<f64>,

    /// Hourly weather elements
    pub weather: Vec<WeatherElement>,
}

#[derive(Debug, Deserialize)]
pub struct DailyTemperature {
    /// Morning temperature.
    pub morn: f64,

    /// Day temperature.
    pub day: f64,

    /// Evening temperature.
    pub eve: f64,

    /// Night temperature.
    pub night: f64,

    /// Min daily temperature.
    pub min: f64,

    /// Max daily temperature.
    pub max: f64,
}

#[derive(Debug, Deserialize)]
pub struct DailyFeelsLikeTemperature {
    /// Morning temperature.
    pub morn: f64,

    /// Day temperature.
    pub day: f64,

    /// Evening temperature.
    pub eve: f64,

    /// Night temperature.
    pub night: f64,
}

/// National weather alerts data from major national weather warning systems
#[derive(Debug, Deserialize)]
pub struct Alert {
    /// Name of the alert source. Please read here the [full list of alert sources](https://openweathermap.org/api/one-call-3#listsource)
    pub sender_name: String,

    /// Alert event name
    pub event: String,

    /// Date and time of the start of the alert, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub start: DateTime<Utc>,

    /// Date and time of the end of the alert, Unix, UTC
    #[serde(with = "ts_seconds")]
    pub end: DateTime<Utc>,

    /// Description of the alert
    pub description: String,

    /// Type of severe weather
    pub tags: Vec<String>,
}
