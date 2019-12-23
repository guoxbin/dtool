use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
use chrono::{NaiveDateTime, Local, FixedOffset, DateTime};
use chrono::offset::TimeZone;

enum Time{
	FixedOffset(DateTime<FixedOffset>),
	Local(DateTime<Local>),
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ts2d").about("Convert timestamp to date")
				.arg(
					Arg::with_name("z")
						.short("z").help("Time zone: 8(CN), 0(UK), etc")
						.takes_value(true)
						.required(false))
				.arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ts2d,
		},
		Command {
			app: SubCommand::with_name("d2ts").about("Convert date to timestamp")
				.arg(
					Arg::with_name("z")
						.short("z").help("Time zone: 8(CN), 0(UK), etc")
						.takes_value(true)
						.required(false))
				.arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: d2ts,
		}
	]
}

fn ts2d(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let timestamp : i64 = input.parse().map_err(|_| "Invalid input")?;

	let timezone = matches.value_of("z");

	let result = match timezone {
		Some(timezone) => {
			let timezone : i32 = timezone.parse().map_err(|_|"Invalid input")?;
			if timezone > 12 || timezone < -12 {
				return Err("Invalid timezone".to_string())
			}
			FixedOffset::east(timezone * 3600).timestamp(timestamp, 0).format("%Y-%m-%d %H:%M:%S").to_string()
		},
		None => {
			Local.timestamp(timestamp, 0).format("%Y-%m-%d %H:%M:%S").to_string()
		}
	};

	Ok(vec![result])
}

fn d2ts(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let timezone = matches.value_of("z");

	let result = parse_standard(&input, timezone)
		.or_else(|_| parse_rfc2822(&input))
		.or_else(|_| parse_rfc3339(&input))?;

	let result = match result{
		Time::FixedOffset(time) => time.timestamp(),
		Time::Local(time) => time.timestamp(),
	};

	let result = format!("{}", result);

	Ok(vec![result])
}

fn parse_standard(input: &str, timezone: Option<&str>) -> Result<Time, String> {

	let time = NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M:%S").map_err(|_| "Invalid input")?;

	let result = match timezone {
		Some(timezone) => {
			let timezone : i32 = timezone.parse().map_err(|_|"Invalid input")?;
			if timezone > 12 || timezone < -12 {
				return Err("Invalid timezone".to_string())
			}
			Time::FixedOffset(FixedOffset::east(timezone * 3600).from_local_datetime(&time).unwrap())
		},
		None => {
			Time::Local(Local.from_local_datetime(&time).unwrap())
		}
	};

	Ok(result)
}

fn parse_rfc2822(input: &str) -> Result<Time, String> {

	DateTime::parse_from_rfc2822(input).map(Time::FixedOffset).map_err(|_| "Invalid input".to_string())
}

fn parse_rfc3339(input: &str) -> Result<Time, String> {

	DateTime::parse_from_rfc3339(input).map(Time::FixedOffset).map_err(|_| "Invalid input".to_string())
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_ts2d() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["ts2d", "-z8", "0"]);
		assert_eq!(ts2d(&matches) , Ok(vec!["1970-01-01 08:00:00".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ts2d", "-z8", "10000"]);
		assert_eq!(ts2d(&matches) , Ok(vec!["1970-01-01 10:46:40".to_string()]));

	}

	#[test]
	fn test_d2ts() {
		let app =  &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["d2ts", "-z8", "1970-01-01 08:00:00"]);
		assert_eq!(d2ts(&matches) , Ok(vec!["0".to_string()]));

		let matches = app.clone().get_matches_from(vec!["d2ts", "-z8", "1970-01-01 10:46:40"]);
		assert_eq!(d2ts(&matches) , Ok(vec!["10000".to_string()]));

		let matches = app.clone().get_matches_from(vec!["d2ts", "Mon, 23 Dec 2019 17:41:26 +0800"]);
		assert_eq!(d2ts(&matches) , Ok(vec!["1577094086".to_string()]));

		let matches = app.clone().get_matches_from(vec!["d2ts", "2019-12-23T17:48:54+08:00"]);
		assert_eq!(d2ts(&matches) , Ok(vec!["1577094534".to_string()]));

	}

}
