// Codes
const CLEAR: [u8;2] = [0,1];
const PARTLY_CLOUDY: u8 = 2;
const OVERCAST: u8 = 3;
const FOG: u8 = 45;
const RIMEFOG: u8 = 48;
const DRIZZLE: [u8; 5] = [51, 53, 55, 56, 57];
const RAIN: [u8; 5] = [61, 63, 65, 66, 67];
const SNOW: [u8; 3] = [77, 85, 86];
const SHOWERS: [u8; 3] = [80, 81, 82];
const THUNDERSTORM: u8 = 95;
const HAILSTORM: [u8; 2] = [96,99];


// Icons
const CLEARICO: char = '';
const PARTLY_CLOUDYICO: char = '杖';
const OVERCASTICO: char = '摒';
const FOGICO: char = '';
const RIMEFOGICO: char = '敖';
const DRIZZLEICO: char = '';
const RAINICO: char = '';
const SNOWICO: char = '';
const SHOWERSICO: char = '';
const THUNDERSTORMICO: char = '';
const HAILICO: char = '';
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
    if CLEAR.contains(&code){
        return CLEARICO;
    } else if DRIZZLE.contains(&code) {
        return DRIZZLEICO;
    } else if RAIN.contains(&code){
        return RAINICO;
    } else if SNOW.contains(&code){
        return SNOWICO;
    } else if SHOWERS.contains(&code) {
        return SHOWERSICO;
    } else if HAILSTORM.contains(&code){
        return HAILICO;
    }

    let co = code.to_owned();
    let output: char = match co {
        PARTLY_CLOUDY => return PARTLY_CLOUDYICO,
        OVERCAST => return OVERCASTICO,
        FOG => return FOGICO,
        RIMEFOG => return RIMEFOGICO,
        THUNDERSTORM => return THUNDERSTORMICO,
        _=> return ERRICO,
    };

    return output;
}

fn text(code: u8) -> String {
    if CLEAR.contains(&code){
        return "Clear".to_string();
    } else if DRIZZLE.contains(&code) {
        return "Drizzle".to_string();
    } else if RAIN.contains(&code){
        return "Raining".to_string();
    } else if SNOW.contains(&code){
        return "Snowing".to_string();
    } else if SHOWERS.contains(&code) {
        return "Showers".to_string();
    } else if HAILSTORM.contains(&code){
        return "Hail".to_string();
    }

    let output: String = match code {
        PARTLY_CLOUDY => return "Partly Cloudy".to_string(),
        OVERCAST => return "Overcast".to_string(),
        FOG => return "Foggy".to_string(),
        RIMEFOG => return "Rime Fog".to_string(),
        THUNDERSTORM => return "Thunderstorms".to_string(),
        _=> return "Weather Error".to_string(),
    };

    return output;
}