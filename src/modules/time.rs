use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::Command;
use std::io;
use std::io::BufRead;
use chrono::{NaiveDateTime, Local};
use chrono::offset::TimeZone;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ts2d").about("Convert timestamp to date").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ts2d,
		},
		Command {
			app: SubCommand::with_name("d2ts").about("Convert date to timestamp").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: d2ts,
		}
	]
}

fn ts2d(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let timestamp : i64 = input.parse().map_err(|_| "Invalid input")?;
	let time = Local.timestamp(timestamp, 0);
	let result = time.format("%Y-%m-%d %H:%M:%S").to_string();

	Ok(vec![result])
}

fn d2ts(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let time = NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M:%S").map_err(|_| "Invalid input")?;
	let time = Local.from_local_datetime(&time).unwrap();
	let result = time.timestamp();
	let result = format!("{}", result);

	Ok(vec![result])
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_ts2d() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["ts2d", "0"]);
		assert_eq!(ts2d(&matches) , Ok(vec!["1970-01-01 08:00:00".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ts2d", "10000"]);
		assert_eq!(ts2d(&matches) , Ok(vec!["1970-01-01 10:46:40".to_string()]));

	}

	#[test]
	fn test_d2ts() {
		let app =  &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["d2ts", "1970-01-01 08:00:00"]);
		assert_eq!(d2ts(&matches) , Ok(vec!["0".to_string()]));

		let matches = app.clone().get_matches_from(vec!["d2ts", "1970-01-01 10:46:40"]);
		assert_eq!(d2ts(&matches) , Ok(vec!["10000".to_string()]));

	}

}
