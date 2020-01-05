use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use urlencoding;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ue").about("URL encode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ue,
			cases: vec![
				Case {
					input: vec!["a+b"].into_iter().map(Into::into).collect(),
					output: vec!["a%2Bb"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
		},
		Command {
			app: SubCommand::with_name("ud").about("URL decode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ud,
			cases: vec![
				Case {
					input: vec!["a%2Bb"].into_iter().map(Into::into).collect(),
					output: vec!["a+b"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
		}
	]
}

fn ue(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = urlencoding::encode(&input);

	Ok(vec![result])
}

fn ud(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = urlencoding::decode(&input).map_err(|_| "Decode failed")?;

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
