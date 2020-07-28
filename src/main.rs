mod backends;

// `error_chain!` can recurse deeply
// #![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
use crate::backends::climacell::CurrentConditions;
use errors::*;

use crate::backends::climacell::get_realtime;
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

	let cc = get_realtime(lat, lon).chain_err(|| "Failed to fetch weather.")?;
	output_current_conditions(cc).chain_err(|| "Failed to print weather.")?;
	Ok(())
}

#[macro_use] extern crate prettytable;

fn output_current_conditions(cc: CurrentConditions) -> Result<()> {
	let mut table = table!(["Text Forecast"]);
	table.add_row(row![bFg-> cc.temp]);
	table.printstd();

	Ok(())
}