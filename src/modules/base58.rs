use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use bs58;
use crate::modules::base::Hex;

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
					desc: "".to_string(),
					input: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					output: vec!["12dvBhvPEPniQmBmgvj4qpJEodT7P"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["0x41e1df5ec0fec39b5cf51d959118819ce5e64643df"].into_iter().map(Into::into).collect(),
					output: vec!["53yFvKp7qazhe1YV6rE5ESr1iofv6"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.1.0".to_string(),
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
					desc: "".to_string(),
					input: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					output: vec!["1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["0x41e1df5ec0fec39b5cf51d959118819ce5e64643df"].into_iter().map(Into::into).collect(),
					output: vec!["TWZWdFSL6Kcn7hfAg6SHiGixUP5efEaBtW"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.1.0".to_string(),
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
					desc: "".to_string(),
					input: vec!["12dvBhvPEPniQmBmgvj4qpJEodT7P"].into_iter().map(Into::into).collect(),
					output: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
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
					desc: "".to_string(),
					input: vec!["1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF"].into_iter().map(Into::into).collect(),
					output: vec!["0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
			],
		},
	]
}

fn h2b58(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input : Vec<u8> = input.parse::<Hex>().map_err(|_| "Convert failed")?.into();

	let result = bs58::encode(input).into_string();

	Ok(vec![result])
}

fn h2b58c(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input : Vec<u8> = input.parse::<Hex>().map_err(|_| "Convert failed")?.into();

	let result = bs58::encode(input).with_check().into_string();

	Ok(vec![result])
}

fn b582h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = bs58::decode(&input).into_vec().map_err(|_| "Convert failed")?;
	let result = Hex::from(input).into();

	Ok(vec![result])
}

fn b58c2h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let input = bs58::decode(&input).with_check(None).into_vec().map_err(|_| "Convert failed")?;
	let result = Hex::from(input).into();

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
