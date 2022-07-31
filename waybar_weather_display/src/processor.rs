// Weather Codes https://open-meteo.com/en/docs
const CLEAR: u8 = 0;
const CLEARICO: char = '';

const PARTLYCLEAR: u8 = 1;
const PARTICO: char = '';

const FOGGY: [u8; 2] = [45, 48];
const FOGICO: char = 'F';


// [0] Partly Cloudy, [1] Overcast
const CLOUDY: [u8; 2] = [2, 3];
const CLOUDICO: char = '';

// [0] Light, [1] Moderate, [2] Dense, [3] Freezing light, [4] Freezing Dense
const DRIZZLE: [u8; 5] = [51, 53, 55, 56, 57];
const DRIZZICO: char = 'D';

// [0] Slight, [1] Moderate, [2] Heavy, [3] Freezing Light, [4] Freezing Dense
const RAIN: [u8; 5] = [61, 63, 65, 66, 67];
const RAINICO: char = '';

// [0] Slight, [1] Moderate, [2] Heavy, [3] Grains
const SNOW: [u8; 4] = [71, 73, 75, 77];
const SNOWICO: char = '';

// Rain Showers
// [0] Slight, [1] Moderate, [2] Heavy
const RSHOWERS: [u8; 3] = [80, 81, 82];
const RSHOWICO: char = '';

// Snow Showers
// [0] Slight, [1] Heavy
const SNSHOWERS: [u8; 2] = [85, 86];
const SNSHOWICO: char = '';

// [0] Slight or Moderate, [1] Slight Hail, [2] Heavy Hail
const THUNDERSTORM: [u8; 3] = [95, 96, 99];
const THUNDERICO: char = 'T';

// Error Icon when code does not return
const ERRICO: char = '';

pub struct Weather {
    pub icon: char,
    pub text: String,
}

pub fn parse_weather(code: u8) -> Weather{
    let ico: char = ico(&code);
    let tex: String = text(code);

    Weather { icon: ico, text: tex }
}

fn ico(code: &u8) -> char {
    if CLEAR == code.to_owned() {
        return CLEARICO;
    } else if PARTLYCLEAR == code.to_owned() {
        return PARTICO;
    } else if FOGGY.contains(&code) {
        return FOGICO;
    } else if CLOUDY.contains(&code) {
        return CLOUDICO;
    } else if DRIZZLE.contains(&code) {
        return DRIZZICO;
    } else if RAIN.contains(&code) {
        return RAINICO;
    } else if SNOW.contains(&code) {
        return SNOWICO;
    } else if RSHOWERS.contains(&code) {
        return RSHOWICO;
    } else if SNSHOWERS.contains(&code) {
        return SNSHOWICO;
    } else if THUNDERSTORM.contains(&code) {
        return THUNDERICO;
    } else {
        return ERRICO;
    }
}

fn text(code: u8) -> String {
    if CLEAR == code {
        return "Clear".to_string();
    } else if PARTLYCLEAR == code {
        return "Partly Cloudy".to_string();
    } else if FOGGY.contains(&code) {
        return "Foggy".to_string();
    } else if CLOUDY.contains(&code) {
        return "Cloudy".to_string();
    } else if DRIZZLE.contains(&code) {
        return "Drizzle".to_string();
    } else if RAIN.contains(&code) {
        return "Raining".to_string();
    } else if SNOW.contains(&code) {
        return "Snowy".to_string();
    } else if RSHOWERS.contains(&code) {
        return "Rain Showers".to_string();
    } else if SNSHOWERS.contains(&code) {
        return "Snow Showers".to_string();
    } else if THUNDERSTORM.contains(&code) {
        return "Thunder Storms".to_string();
    } else {
        return "Weather Error".to_string();
    }
}
