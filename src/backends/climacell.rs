// use chrono::DateTime;
use chrono::Utc;
use chrono::DateTime;
use error_chain::error_chain;
use serde::Deserialize;
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
           Unit::F    => write!(f, "F"),
           Unit::C    => write!(f, "C"),
           Unit::MPH  => write!(f, "mph"),
           Unit::MS   => write!(f, "m/s"),
           Unit::HPA  => write!(f, "hPa"),
           Unit::INHG => write!(f, "inHg"),
       }
   }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "value", rename_all(deserialize = "PascalCase"))]
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
           PrecipType::None         => write!(f, "None"),
           PrecipType::Rain         => write!(f, "Rain"),
           PrecipType::Snow         => write!(f, "Snow"),
           PrecipType::IcePellets   => write!(f, "Ice Pellets"),
           PrecipType::FreezingRain => write!(f, "Freezing Rain"),
       }
   }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "value", rename_all(deserialize = "PascalCase"))]
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
           MoonPhase::New            => write!(f, "🌑"),
           MoonPhase::WaxingCrescent => write!(f, "🌒"),
           MoonPhase::FirstQuarter   => write!(f, "🌓"),
           MoonPhase::WaxingGibbous  => write!(f, "🌔"),
           MoonPhase::Full           => write!(f, "🌕"),
           MoonPhase::WaningGibbous  => write!(f, "🌖"),
           MoonPhase::LastQuarter    => write!(f, "🌗"),
           MoonPhase::WaningCrescent => write!(f, "🌘"),
       }
   }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "value", rename_all(deserialize = "PascalCase"))]
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
           WeatherCode::FreezingRainHeavy => write!(f, "🍦"),
           WeatherCode::FreezingRain => write!(f, "🍦"),
           WeatherCode::FreezingRainLight => write!(f, "🍦"),
           WeatherCode::FreezingDrizzle => write!(f, "🍦"),
           WeatherCode::IcePelletsHeavy => write!(f, "🌨"),
           WeatherCode::IcePellets => write!(f, "🌨"),
           WeatherCode::IcePelletsLight => write!(f, "🌨"),
           WeatherCode::SnowHeavy => write!(f, "❄️"),
           WeatherCode::Snow => write!(f, "❄️"),
           WeatherCode::SnowLight => write!(f, "❄️"),
           WeatherCode::Flurries => write!(f, "❄️"),
           WeatherCode::Tstorm => write!(f, "⛈"),
           WeatherCode::RainHeavy => write!(f, "🌧"),
           WeatherCode::Rain => write!(f, "🌧"),
           WeatherCode::RainLight => write!(f, "🌦"),
           WeatherCode::Drizzle => write!(f, "🌧"),
           WeatherCode::FogLight => write!(f, "🌫"),
           WeatherCode::Fog => write!(f, "🌫"),
           WeatherCode::Cloudy => write!(f, "☁️"),
           WeatherCode::MostlyCloudy => write!(f, "🌥"),
           WeatherCode::PartlyCloudy => write!(f, "⛅️"),
           WeatherCode::MostlyClear => write!(f, "🌤"),
           WeatherCode::Clear => write!(f, "☀️"),
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
		write!(f, "{}° {}", self.value, self.units)
	}
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PercentLike {
	pub value: f64,
}
impl fmt::Display for PercentLike {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}%", self.value)
	}
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SpeedLike {
	pub value: f64,
	pub units: Unit,
}
impl fmt::Display for SpeedLike {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.value, self.units)
	}
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PressureLike {
	pub value: f64,
	pub units: Unit,
}
impl fmt::Display for PressureLike {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.value, self.units)
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
		if (d >= 0.0 && d < 11.5) || (d <= 180.0 && d > 168.0) {
			write!(f, "{}° N", self.degrees)
		} else if d >= 11.5 && d < 34.0 {
			write!(f, "{}° NE", self.degrees)
		} else if d >= 34.0 && d <= 56.5 {
			write!(f, "{}° E", self.degrees)
		} else if d >= 56.5 && d <= 79.0 {
			write!(f, "{}° SE", self.degrees)
		} else if d >= 79.0 && d <= 101.5 {
			write!(f, "{}° S", self.degrees)
		} else if d >= 101.5 && d < 123.0 {
			write!(f, "{}° SW", self.degrees)
		} else if d >= 123.0 && d <= 145.5 {
			write!(f, "{}° W", self.degrees)
		} else { // 145.5..=168.0 
			write!(f, "{}° NW", self.degrees)
		}
	}
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CurrentConditions {
	pub lat: f64,
	pub lon: f64,
	#[serde(flatten)]
	pub observation_time: DateTime<Utc>,
	pub temp: TempLike,
	pub feels_like: TempLike,
	pub dewpoint: TempLike,
	#[serde(flatten)]
	pub humidity: PercentLike,
	pub wind_speed: SpeedLike,
	pub wind_direction: CompassLike,
	pub wind_gust: SpeedLike,
	pub baro_pressure: PressureLike,
	pub precipitation_type: PrecipType,
	#[serde(flatten)]
	pub sunrise: DateTime<Utc>,
	#[serde(flatten)]
	pub sunset: DateTime<Utc>,
	pub cloud_cover: PercentLike,
	pub moon_phase: MoonPhase,
	pub weather_code: WeatherCode,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Test(chrono::format::ParseError);
    }
}


pub fn get_realtime(lat: f64, lon: f64) -> Result<CurrentConditions> {
	let request_url = format!("https://api.climacell.co/v3/weather/realtime?lat={lat}&lon={lon}&unit_system=us&fields=dewpoint%2Cfeels_like%2Ctemp&apikey={apikey}",
                              lat = lat,
                              lon = lon,
                              apikey = "TODO");
    println!("{}", request_url);
    reqwest::blocking::get(&request_url)?.json().chain_err(|| "Failed to get response as JSON.")
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_deserialize_realtime() -> Result<()> {
    	let json = fs::read_to_string("realtime.json")?;
    	let rt: CurrentConditions = serde_json::from_str(&json).chain_err(|| "Test failed to convert to JSON.")?;
    	println!("RT {:?}", rt);
    	let other = CurrentConditions {
    		lat: 38.860409, 
    		lon: -77.32479, 
    		temp: TempLike { value: 85.55, units: Unit::F }, 
    		feels_like: TempLike { value: 91.51, units: Unit::F },
    		dewpoint: TempLike { value: 71.83, units: Unit::F },
    		observation_time: "2020-07-28T02:00:03.981Z".parse::<DateTime<Utc>>()?,
    		baro_pressure: PressureLike { value: 29.8234, units: Unit::INHG },
    		cloud_cover: PercentLike { value: 0.0 },
    		humidity: PercentLike { value: 63.69 },
    		moon_phase: MoonPhase::FirstQuarter,
    		precipitation_type: PrecipType::None,
    		weather_code: WeatherCode::Clear,
    		wind_direction: CompassLike { degrees: 219.25 },
    		wind_gust: SpeedLike { value: 13.0, units: Unit::MPH },
    		wind_speed: SpeedLike { value: 6.15, units: Unit::MPH },
    		sunrise: "2020-07-28T10:07:34.007Z".parse::<DateTime<Utc>>()?,
    		sunset: "2020-07-29T00:23:59.494Z".parse::<DateTime<Utc>>()?
    	};

    	assert_eq!(rt, other);

    	Ok(())
    }
}