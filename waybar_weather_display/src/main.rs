// My Modules
mod args;
mod api;
mod processor;

// My Module Use Statements
use args::Args;
use processor::{parse_weather, Weather};

// External Module Use Statements
use clap::Parser;

// TODO Create Configuration File Parser
// Params for config, All Icons configurable, Messages for weather configurable, Latitude, Longitude, City Name, What to include in weather data
fn main() {
    let args:Args = Args::parse();
    let code: u8 = api::weather_code(args).unwrap() as u8;
    let weather: Weather = parse_weather(code);
    println!("{{\"text\":\"{} {}\", \"alt\":\"{}\"}}", weather.text, weather.icon, weather.icon)
}
