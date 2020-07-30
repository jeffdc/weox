mod climacell;
mod weatherdata;
use crate::climacell::{get_forecast, get_realtime};
use crate::weatherdata::{CurrentConditions, Unit};

// `error_chain!` can recurse deeply
// #![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
use chrono::{DateTime, Local};
use colored::*;
use errors::*;
use prettytable::{format, row, Cell, Row, Table};
use structopt::StructOpt;

// We'll put our errors in an `errors` module, and other modules in this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
	// Create the Error, ErrorKind, ResultExt, and Result types
	error_chain! {
		foreign_links {
			   Io(std::io::Error);
			HttpRequest(reqwest::Error);
		}
	}
}

#[derive(StructOpt)]
struct Cli {
	location: String,
	// forecast otherwise current
	// short and long flags (-f, --forecast) will be deduced from the field's name
	#[structopt(short, long)]
	forecast: bool,
}

// #[tokio::main]
fn main() {
	if let Err(ref e) = run() {
		println!("error: {}", e);

		for e in e.iter().skip(1) {
			println!("caused by: {}", e);
		}

		// The backtrace is not always generated. Try to run this  with `RUST_BACKTRACE=1`.
		if let Some(backtrace) = e.backtrace() {
			println!("backtrace: {:?}", backtrace);
		}

		::std::process::exit(1);
	}
}

fn run() -> Result<()> {
	let args = Cli::from_args();
	// for now hard-coded, deferring geo lookup code to later
	let _loc = args.location;
	let lat = 38.860409;
	let lon = -77.32479;
	
	if args.forecast {
		let f = get_forecast(lat, lon, true).chain_err(|| "Failed to fetch weather.")?;
		println!("{:?}", f);
	} else {
		let cc = get_realtime(lat, lon, false).chain_err(|| "Failed to fetch weather.")?;
		output_current_conditions(cc).chain_err(|| "Failed to print weather.")?;	
	}
	Ok(())
}

#[macro_use]
extern crate prettytable;

fn output_current_conditions(cc: CurrentConditions) -> Result<()> {
	println!(
		"\n{} {}:\n",
		"Current Conditions for".green(),
		"Fairfax, VA".bright_yellow().bold()
	);
	let mut table = Table::new();
	let format = format::FormatBuilder::new()
		.column_separator('┊')
		.separators(
			&[
				format::LinePosition::Title,
				format::LinePosition::Bottom,
				format::LinePosition::Intern,
			],
			format::LineSeparator::new('┄', '┄', ' ', ' '),
		)
		.padding(1, 1)
		.build();
	table.set_format(format);

	table.set_titles(row![
		cb->"Weather",
		cb->"Temp",
		cb->"Feels Like",
		cb->"Humidity",
		cb->"Dew Pt",
		cb->"Pressure",
		cb->"Moon",
		cb->"Sunrise",
		cb->"Sunset",
		cb->"Wind",
		cb->"Direction",
		cb->"Gust",
		cb->"Precip Type",
		cb->"Cloud Cover"
	]);
	let sunrise: DateTime<Local> = DateTime::from(cc.sunrise.dt);
	let sunset: DateTime<Local> = DateTime::from(cc.sunset.dt);
	// // due to wonkiness in the Climacell API dewpoint is not available in forecast data, sigh...
	// let dp = if cc.dewpoint.value < 0.0 {
	// 	compute_dewpoint(cc.temp.value, cc.humidity.value)
	// } else {
	// 	cc.dewpoint.value
	// };

	table.add_row(Row::new(vec![
		Cell::new(&cc.weather_code.to_string()).style_spec("c"),
		Cell::new(&cc.temp.to_string()).style_spec(temp_color(cc.temp.value, cc.temp.units == Unit::C)),
		Cell::new(&cc.feels_like.to_string())
			.style_spec(temp_color(cc.feels_like.value, cc.feels_like.units == Unit::C)),
		Cell::new(&cc.humidity.to_string()).style_spec("c"),
		Cell::new(&cc.dewpoint.to_string()).style_spec(dewpoint_color(cc.dewpoint.value, cc.dewpoint.units == Unit::C)),
		Cell::new(&cc.baro_pressure.to_string()).style_spec("c"),
		Cell::new(&cc.moon_phase.to_string()).style_spec("c"),
		Cell::new(&sunrise.format("%H:%M:%S").to_string()).style_spec("c"),
		Cell::new(&sunset.format("%H:%M:%S").to_string()).style_spec("c"),
		Cell::new(&cc.wind_speed.to_string()).style_spec("c"),
		Cell::new(&cc.wind_direction.to_string()).style_spec("c"),
		Cell::new(&cc.wind_gust.to_string()).style_spec("c"),
		Cell::new(&cc.precipitation_type.to_string()).style_spec("c"),
		Cell::new(&cc.cloud_cover.to_string()).style_spec("c"),
	]));
	table.printstd();

	Ok(())
}

fn dewpoint_color(dp: f64, celsius: bool) -> &'static str {
	let convert = |n| -> f64 {
		if celsius {
			n * 1.8 + 32.0
		} else {
			n
		}
	};

	if dp > convert(80.0) {
		"FRc"
	} else if dp > convert(74.0) {
		"Frc"
	} else if dp > convert(69.0) {
		"FYc"
	} else if dp > convert(64.0) {
		"Fyc"
	} else if dp > convert(59.0) {
		"FGc"
	} else if dp > convert(54.0) {
		"Fgc"
	} else {
		"Fbc"
	}
}

fn temp_color(dp: f64, celsius: bool) -> &'static str {
	let convert = |n| -> f64 {
		if celsius {
			n * 1.8 + 32.0
		} else {
			n
		}
	};

	if dp > convert(95.0) {
		"FRc"
	} else if dp > convert(85.0) {
		"Frc"
	} else if dp > convert(75.0) {
		"FYc"
	} else if dp > convert(65.0) {
		"Fyc"
	} else if dp > convert(55.0) {
		"FGc"
	} else if dp > convert(45.0) {
		"Fgc"
	} else if dp > convert(35.0) {
		"Fcc"
	} else {
		"Fbc"
	}
}
