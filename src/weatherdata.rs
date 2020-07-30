use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Debug, PartialEq)]
pub enum Unit {
    F,
    C,
    #[serde(alias = "mph")]
    MPH,
    #[serde(alias = "m/s")]
    MS,
    #[serde(alias = "hPa")]
    HPA,
    #[serde(alias = "inHg")]
    INHG,
}
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unit::F => write!(f, "F"),
            Unit::C => write!(f, "C"),
            Unit::MPH => write!(f, "mph"),
            Unit::MS => write!(f, "m/s"),
            Unit::HPA => write!(f, "hPa"),
            Unit::INHG => write!(f, "inHg"),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "value", rename_all(deserialize = "snake_case"))]
pub enum PrecipType {
    None,
    Rain,
    Snow,
    IcePellets,
    FreezingRain,
}
impl fmt::Display for PrecipType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrecipType::None => write!(f, "None"),
            PrecipType::Rain => write!(f, "Rain"),
            PrecipType::Snow => write!(f, "Snow"),
            PrecipType::IcePellets => write!(f, "Ice Pellets"),
            PrecipType::FreezingRain => write!(f, "Freezing Rain"),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "value", rename_all(deserialize = "snake_case"))]
pub enum MoonPhase {
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    Full,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}
impl fmt::Display for MoonPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoonPhase::New => write!(f, "🌑"),
            MoonPhase::WaxingCrescent => write!(f, "🌒"),
            MoonPhase::FirstQuarter => write!(f, "🌓"),
            MoonPhase::WaxingGibbous => write!(f, "🌔"),
            MoonPhase::Full => write!(f, "🌕"),
            MoonPhase::WaningGibbous => write!(f, "🌖"),
            MoonPhase::LastQuarter => write!(f, "🌗"),
            MoonPhase::WaningCrescent => write!(f, "🌘"),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "value", rename_all(deserialize = "snake_case"))]
pub enum WeatherCode {
    FreezingRainHeavy,
    FreezingRain,
    FreezingRainLight,
    FreezingDrizzle,
    IcePelletsHeavy,
    IcePellets,
    IcePelletsLight,
    SnowHeavy,
    Snow,
    SnowLight,
    Flurries,
    Tstorm,
    RainHeavy,
    Rain,
    RainLight,
    Drizzle,
    FogLight,
    Fog,
    Cloudy,
    MostlyCloudy,
    PartlyCloudy,
    MostlyClear,
    Clear,
}
impl fmt::Display for WeatherCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeatherCode::FreezingRainHeavy => write!(f, "🍦 Heavy Freezing Rain"),
            WeatherCode::FreezingRain => write!(f, "🍦 Freezing Rain"),
            WeatherCode::FreezingRainLight => write!(f, "🍦 Light Freezing Rain"),
            WeatherCode::FreezingDrizzle => write!(f, "🍦 Freezing Drizzle"),
            WeatherCode::IcePelletsHeavy => write!(f, "🌨 Heavy Ice Pellets"),
            WeatherCode::IcePellets => write!(f, "🌨 Ice Pellets"),
            WeatherCode::IcePelletsLight => write!(f, "🌨 Light Ice Pellets"),
            WeatherCode::SnowHeavy => write!(f, "❄️ Heavy Snow"),
            WeatherCode::Snow => write!(f, "❄️ Snow"),
            WeatherCode::SnowLight => write!(f, "❄️ Light Snow"),
            WeatherCode::Flurries => write!(f, "❄️ Flurries"),
            WeatherCode::Tstorm => write!(f, "⛈ T-Storm"),
            WeatherCode::RainHeavy => write!(f, "🌧 Heavy Rain"),
            WeatherCode::Rain => write!(f, "🌧 Rain"),
            WeatherCode::RainLight => write!(f, "🌦 Light Rain"),
            WeatherCode::Drizzle => write!(f, "🌧 Drizzle"),
            WeatherCode::FogLight => write!(f, "🌫 Light Fog"),
            WeatherCode::Fog => write!(f, "🌫 Fog"),
            WeatherCode::Cloudy => write!(f, "☁️ Cloudy"),
            WeatherCode::MostlyCloudy => write!(f, "🌥 Mostly Cloudy"),
            WeatherCode::PartlyCloudy => write!(f, "⛅️ Partly Cloudy"),
            WeatherCode::MostlyClear => write!(f, "🌤 Mostly Clear"),
            WeatherCode::Clear => write!(f, "🌞 Clear"),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TempLike {
    pub value: f64,
    pub units: Unit,
}
impl fmt::Display for TempLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}° {}", self.value, self.units)
    }
}
impl Default for TempLike {
    fn default() -> Self {
        TempLike {
            value: -1.0,
            units: Unit::F,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PercentLike {
    pub value: f64,
}
impl fmt::Display for PercentLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}%", self.value)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SpeedLike {
    pub value: f64,
    pub units: Unit,
}
impl fmt::Display for SpeedLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} {}", self.value, self.units)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PressureLike {
    pub value: f64,
    pub units: Unit,
}
impl fmt::Display for PressureLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, self.units)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TimeLike {
    #[serde(alias = "value")]
    pub dt: DateTime<Utc>,
}
impl fmt::Display for TimeLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let local: DateTime<Local> = DateTime::from(self.dt);
        write!(f, "{}", local)
    }
}
impl Default for TimeLike {
    fn default() -> Self {
        TimeLike {
            dt: Utc::now(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CompassLike {
    #[serde(alias = "value")]
    pub degrees: f64,
}
impl fmt::Display for CompassLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // for some reason can not match on floats without hideous compiler warning.
        // supposedly there is an explanation here but it seems still up in the air, sigh.
        // https://github.com/rust-lang/rust/issues/41620
        let d = self.degrees;
        if (d >= 0.0 && d < 22.5) || (d <= 360.0 && d > 337.5) {
            write!(f, "⬇ N")
        } else if d >= 22.5 && d < 67.5 {
            write!(f, "↙  NE")
        } else if d >= 67.5 && d <= 112.5 {
            write!(f, "⬅ E")
        } else if d >= 112.5 && d <= 157.5 {
            write!(f, "↖ SE")
        } else if d >= 157.5 && d <= 202.5 {
            write!(f, "⬆ S")
        } else if d >= 202.5 && d < 247.5 {
            write!(f, "↗ SW")
        } else if d >= 247.5 && d <= 292.5 {
            write!(f, "➡ W")
        } else {
            // 292.5..=337.5
            write!(f, "↘  NW")
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CurrentConditions {
    pub lat: f64,
    pub lon: f64,
    pub observation_time: TimeLike,
    pub temp: TempLike,
    pub feels_like: TempLike,
    #[serde(default)]
    pub dewpoint: TempLike,
    pub humidity: PercentLike,
    pub wind_speed: SpeedLike,
    pub wind_direction: CompassLike,
    pub wind_gust: SpeedLike,
    pub baro_pressure: PressureLike,
    pub precipitation_type: PrecipType,
    pub sunrise: TimeLike,
    pub sunset: TimeLike,
    pub cloud_cover: PercentLike,
    pub moon_phase: MoonPhase,
    pub weather_code: WeatherCode,
}

// Computes an approximation of the dewpoint given the temp and the relative humidity.
pub fn compute_dewpoint(t: f64, rh: f64) -> f64 {
    // http://bmcnoldy.rsmas.miami.edu/Humidity.html
    243.04 * ((rh / 100.0).ln() + ((17.625 * t) / (243.04 + t)))
        / (17.625 - (rh / 100.0).ln() - ((17.625 * t) / (243.04 + t)))
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastLoad {
    days: HashMap<String, ForecastItem>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastItem {
    obs: Vec<HashMap<String, VUTuple>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct VUTuple {
    value: f64,
    units: Unit
}

#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct ForecastTemp {
    observation_time: TimeLike,
    min: TempLike,
    max: TempLike,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastFeelsLike {
    observation_time: TimeLike,
    min: TempLike,
    max: TempLike,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastHumidity {
    observation_time: TimeLike,
    min: TempLike,
    max: TempLike,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastBaroPressure {
    observation_time: TimeLike,
    min: PressureLike,
    max: PressureLike,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastWindSpeed {
    observation_time: TimeLike,
    min: SpeedLike,
    max: SpeedLike,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastWindDirection {
    observation_time: TimeLike,
    min: CompassLike,
    max: CompassLike,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ForecastDay {
    temp: ForecastTemp,
    feels_like: ForecastFeelsLike,
    humidity: ForecastHumidity,
    baro_pressure: ForecastBaroPressure,
    wind_speed: ForecastWindSpeed,
    wind_direction: ForecastWindDirection,
    weather_code: WeatherCode,
    #[serde(skip)]
    dewpoint: ForecastTemp,
    observation_time: TimeLike,
    lat: f64,
    lon: f64,
}
// impl fmt::Display for ForecastDay {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, )
//     }
// }