use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use hex;
use base64;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2b64").about("Convert hex to base64").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2b64,
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec!["0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["YWJj"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
			],
		},
		Command {
			app: SubCommand::with_name("b642h").about("Convert base64 to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: b642h,
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec!["YWJj"].into_iter().map(Into::into).collect(),
					output: vec!["0x616263"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
			],
		},
	]
}

fn h2b64(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let result = base64::encode(&input);

	Ok(vec![result])
}

fn b642h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = base64::decode(&input).map_err(|_| "Convert failed")?;
	let result = hex::encode(result);
	let result = format!("0x{}", result);

	Ok(vec![result])
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
