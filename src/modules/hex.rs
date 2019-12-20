use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
use hex;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2s").about("Convert hex to string").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2s,
		},
		Command {
			app: SubCommand::with_name("s2h").about("Convert string to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: s2h,
		},
		Command {
			app: SubCommand::with_name("b2h").about("Convert binary to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: b2h,
		}
	]
}

fn h2s(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;
	let input = input.trim_start_matches("0x");

	let result = hex::decode(input).map_err(|_| "Convert failed")?;
	let result = String::from_utf8(result).map_err(|_| "Convert failed")?;

	Ok(vec![result])
}

fn s2h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = hex::encode(input);
	let result = "0x".to_string() + &result;

	Ok(vec![result])
}

fn b2h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_bytes(matches)?;

	let result = hex::encode(input);
	let result = "0x".to_string() + &result;

	Ok(vec![result])
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_h2s() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["h2s", "0x61626364"]);
		assert_eq!(h2s(&matches) , Ok(vec!["abcd".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2s", "0x21"]);
		assert_eq!(h2s(&matches) , Ok(vec!["!".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2s", "0x6162636465666768696a6b6c6d6e6f707172737475767778797a6162636465666768696a6b6c6d6e6f707172737475767778797a"]);
		assert_eq!(h2s(&matches) , Ok(vec!["abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string()]));

	}

	#[test]
	fn test_s2h() {
		let app =  &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["s2h", "abcd"]);
		assert_eq!(s2h(&matches) , Ok(vec!["0x61626364".to_string()]));

		let matches = app.clone().get_matches_from(vec!["s2h", "!"]);
		assert_eq!(s2h(&matches) , Ok(vec!["0x21".to_string()]));

		let matches = app.clone().get_matches_from(vec!["s2h", "abcdefg"]);
		assert_eq!(s2h(&matches) , Ok(vec!["0x61626364656667".to_string()]));

	}

}
