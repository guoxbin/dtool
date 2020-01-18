use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Module};
use urlencoding;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "URL encode / decode".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ue").about("URL encode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ue,
			cases: vec![

			],
		},
		Command {
			app: SubCommand::with_name("ud").about("URL decode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ud,
			cases: vec![

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

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			("ue",
			 vec![
				 Case {
					 desc: "".to_string(),
					 input: vec!["a+b"].into_iter().map(Into::into).collect(),
					 output: vec!["a%2Bb"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.1.0".to_string(),
				 },
			 ]),
			("ud",
			 vec![
				 Case {
					 desc: "".to_string(),
					 input: vec!["a%2Bb"].into_iter().map(Into::into).collect(),
					 output: vec!["a+b"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.1.0".to_string(),
				 },
			 ]),
		].into_iter().collect()
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::modules::base::test::test_module;

	#[test]
	fn test_cases() {
		test_module(module());
	}

}
