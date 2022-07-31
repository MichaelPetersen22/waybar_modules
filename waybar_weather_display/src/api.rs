// JSON Parser, API Request Tool
use serde_json;
use reqwest;

// Arguments Data Structure so that function consumes args
use super::args::Args;

pub fn weather_code(data: Args) -> serde_json::Result<f64> {
    let default: f64 = 100.0;
    let fetched_data: Result<String, reqwest::Error> = get_weather(data);
    let json: serde_json::Value = serde_json::from_str(fetched_data.unwrap().as_str())?;
    let parsed_data: Option<f64>  = json["current_weather"]["weathercode"].as_f64();
    
    if parsed_data != None {
        Ok(parsed_data.unwrap())
    } else {
        Ok(default)
    }
}


fn get_weather(data: Args) -> Result<String, ::reqwest::Error>{
    let body = reqwest::blocking::get(format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=weathercode&current_weather=true", data.latitude, data.longitude))?
    .text()?;
    Ok(body)
}