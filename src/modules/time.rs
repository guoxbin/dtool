use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
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
					Arg::with_name("TIMEZONE")
						.long("timezone")
						.short("z").help("Time zone\n8: CN\n0: UK\netc")
						.takes_value(true)
						.required(false))
				.arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ts2d,
			cases: vec![
				Case {
					input: vec!["-z8", "0"].into_iter().map(Into::into).collect(),
					output: vec!["1970-01-01 08:00:00"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
				Case {
					input: vec!["-z8", "10000"].into_iter().map(Into::into).collect(),
					output: vec!["1970-01-01 10:46:40"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
		},
		Command {
			app: SubCommand::with_name("d2ts").about("Convert date to timestamp")
				.arg(
					Arg::with_name("TIMEZONE")
						.long("timezone")
						.short("z").help("Time zone\n8: CN\n0: UK\netc")
						.takes_value(true)
						.required(false))
				.arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: d2ts,
			cases: vec![
				Case {
					input: vec!["-z8", "1970-01-01 08:00:00"].into_iter().map(Into::into).collect(),
					output: vec!["0"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
				Case {
					input: vec!["-z8", "1970-01-01 10:46:40"].into_iter().map(Into::into).collect(),
					output: vec!["10000"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
				Case {
					input: vec!["Mon, 23 Dec 2019 17:41:26 +0800"].into_iter().map(Into::into).collect(),
					output: vec!["1577094086"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
				Case {
					input: vec!["2019-12-23T17:48:54+08:00"].into_iter().map(Into::into).collect(),
					output: vec!["1577094534"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
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
	use crate::modules::base::test::test_commands;

	#[test]
	fn test_cases() {
		test_commands(&commands());
	}

}
