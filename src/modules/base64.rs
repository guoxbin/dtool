use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Hex / base64 conversion".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2b64")
				.about("Convert hex to base64")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: h2b64,
		},
		Command {
			app: SubCommand::with_name("b642h")
				.about("Convert base64 to hex")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: b642h,
		},
	]
}

use base64::{engine::general_purpose, Engine as _};

fn h2b64(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Convert failed")?.into();

	let result = general_purpose::STANDARD.encode(&input);

	Ok(vec![result])
}

fn b642h(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let result = general_purpose::STANDARD
		.decode(&input)
		.map_err(|_| "Convert failed")?;
	let result = Hex::from(result).into();

	Ok(vec![result])
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			(
				"h2b64",
				vec![Case {
					desc: "".to_string(),
					input: vec!["0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["YWJj"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				}],
			),
			(
				"b642h",
				vec![Case {
					desc: "".to_string(),
					input: vec!["YWJj"].into_iter().map(Into::into).collect(),
					output: vec!["0x616263"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				}],
			),
		]
		.into_iter()
		.collect()
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
