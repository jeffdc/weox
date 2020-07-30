use crate::weatherdata::*;

#[cfg(test)] // needed to prevent compiler warning during non test builds
use chrono::{DateTime, Utc};
use error_chain::error_chain;
use std::{env, fs};

error_chain! {
	foreign_links {
		Io(std::io::Error);
		HttpRequest(reqwest::Error);
		Test(chrono::format::ParseError);
		JsonError(serde_json::error::Error);
		EnvError(std::env::VarError);
	}
}

pub fn get_realtime(lat: f64, lon: f64, file: bool) -> Result<CurrentConditions> {
	if file {
		load_cc_from_json_file()
	} else {
		let apikey = env::var("CLIMACELL_API_KEY")?;
		let request_url = format!("https://api.climacell.co/v3/weather/realtime?lat={lat}&lon={lon}&unit_system=us&fields=dewpoint%2Cfeels_like%2Ctemp%2Chumidity%2Cwind_speed%2Cwind_direction%2Cwind_gust%2Cbaro_pressure%2Cprecipitation_type%2Csunrise%2Csunset%2Ccloud_cover%2Cmoon_phase%2Cweather_code&apikey={apikey}",
								  lat = lat,
								  lon = lon,
								  apikey = apikey);
		reqwest::blocking::get(&request_url)?
			.json()
			.chain_err(|| "Failed to get response as JSON.")
	}
}

pub fn get_forecast(lat: f64, lon: f64, file: bool) -> Result<ForecastLoad> {
	if file {
		load_forecast_from_json_file()
	} else {
		unimplemented!()
	}
}

#[allow(dead_code)]
pub fn load_cc_from_json_file() -> Result<CurrentConditions> {
	let json = fs::read_to_string("realtime.json")?;
	serde_json::from_str(&json).chain_err(|| "Test failed to convert to JSON.")
}

#[allow(dead_code)]
pub fn load_forecast_from_json_file() -> Result<ForecastLoad> {
	let json = fs::read_to_string("forecast.json")?;
	serde_json::from_str(&json).chain_err(|| "Test failed to convert to JSON.")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_deserialize_realtime() -> Result<()> {
		let other = CurrentConditions {
			lat: 38.860409,
			lon: -77.32479,
			temp: TempLike {
				value: 85.55,
				units: Unit::F,
			},
			feels_like: TempLike {
				value: 91.51,
				units: Unit::F,
			},
			dewpoint: TempLike {
				value: 71.83,
				units: Unit::F,
			},
			observation_time: TimeLike {
				dt: "2020-07-28T02:00:03.981Z".parse::<DateTime<Utc>>()?,
			},
			baro_pressure: PressureLike {
				value: 29.8234,
				units: Unit::INHG,
			},
			cloud_cover: PercentLike { value: 0.0 },
			humidity: PercentLike { value: 63.69 },
			moon_phase: MoonPhase::FirstQuarter,
			precipitation_type: PrecipType::None,
			weather_code: WeatherCode::Clear,
			wind_direction: CompassLike { degrees: 219.25 },
			wind_gust: SpeedLike {
				value: 13.0,
				units: Unit::MPH,
			},
			wind_speed: SpeedLike {
				value: 6.15,
				units: Unit::MPH,
			},
			sunrise: TimeLike {
				dt: "2020-07-28T10:07:34.007Z".parse::<DateTime<Utc>>()?,
			},
			sunset: TimeLike {
				dt: "2020-07-29T00:23:59.494Z".parse::<DateTime<Utc>>()?,
			},
		};

		assert_eq!(load_cc_from_json_file()?, other);

		Ok(())
	}
}
