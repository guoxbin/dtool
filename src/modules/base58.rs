use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use hex;
use bs58;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2b58").about("Convert hex to base58").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2b58,
			cases: vec![
				Case {
					input: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					output: vec!["12dvBhvPEPniQmBmgvj4qpJEodT7P"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
				Case {
					input: vec!["0x41e1df5ec0fec39b5cf51d959118819ce5e64643df"].into_iter().map(Into::into).collect(),
					output: vec!["53yFvKp7qazhe1YV6rE5ESr1iofv6"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
				}
			],
		},
		Command {
			app: SubCommand::with_name("h2b58c").about("Convert hex to base58 check").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2b58c,
			cases: vec![
				Case {
					input: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					output: vec!["1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
				Case {
					input: vec!["0x41e1df5ec0fec39b5cf51d959118819ce5e64643df"].into_iter().map(Into::into).collect(),
					output: vec!["TWZWdFSL6Kcn7hfAg6SHiGixUP5efEaBtW"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
				},
			],
		},
		Command {
			app: SubCommand::with_name("b582h").about("Convert base58 to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: b582h,
			cases: vec![
				Case {
					input: vec!["12dvBhvPEPniQmBmgvj4qpJEodT7P"].into_iter().map(Into::into).collect(),
					output: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
		},
		Command {
			app: SubCommand::with_name("b58c2h").about("Convert base58 check to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: b58c2h,
			cases: vec![
				Case {
					input: vec!["1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF"].into_iter().map(Into::into).collect(),
					output: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
		},
	]
}

fn h2b58(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let result = bs58::encode(input).into_string();

	Ok(vec![result])
}

fn h2b58c(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let result = bs58::encode(input).with_check().into_string();

	Ok(vec![result])
}

fn b582h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = bs58::decode(&input).into_vec().map_err(|_| "Convert failed")?;
	let result = hex::encode(input);
	let result = format!("0x{}", result);

	Ok(vec![result])
}

fn b58c2h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = bs58::decode(&input).with_check(None).into_vec().map_err(|_| "Convert failed")?;
	let result = hex::encode(input);
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
