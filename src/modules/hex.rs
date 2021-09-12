use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use regex::Regex;
use std::io;
use std::io::Write;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Hex / UTF-8 string / binary / byte array conversion".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2s")
				.about("Convert hex to UTF-8 string")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: h2s,
		},
		Command {
			app: SubCommand::with_name("s2h")
				.about("Convert UTF-8 string to hex")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: s2h,
		},
		Command {
			app: SubCommand::with_name("h2b")
				.about("Convert hex to binary")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: h2b,
		},
		Command {
			app: SubCommand::with_name("b2h")
				.about("Convert binary to hex")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: b2h,
		},
		Command {
			app: SubCommand::with_name("h2a")
				.about("Convert hex to byte array")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: h2a,
		},
		Command {
			app: SubCommand::with_name("a2h")
				.about("Convert byte array to hex")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: a2h,
		},
	]
}

fn h2s(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;
	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Convert failed")?.into();

	let result = String::from_utf8(input).map_err(|_| "Not UTF-8")?;

	Ok(vec![result])
}

fn s2h(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches).map_err(|_| "Not UTF-8")?;

	let input = input.as_bytes().to_vec();
	let result: String = Hex::from(input).into();

	Ok(vec![result])
}

fn h2b_inner(matches: &ArgMatches) -> Result<Vec<u8>, String> {
	let input = base::input_string(matches)?;
	let result: Vec<u8> = input.parse::<Hex>().map_err(|_| "Convert failed")?.into();

	Ok(result)
}

fn h2b(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let result = h2b_inner(matches)?;

	io::stdout()
		.write_all(&result)
		.map_err(|_| "Convert failed")?;

	Ok(vec![])
}

fn b2h(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_bytes(matches)?;

	let result: String = Hex::from(input).into();

	Ok(vec![result])
}

fn h2a(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;
	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Convert failed")?.into();

	let result = input
		.into_iter()
		.map(|x| format!("{}", x))
		.collect::<Vec<String>>()
		.join(", ");
	let result = format!("[{}]", result);

	Ok(vec![result])
}

fn a2h(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches).map_err(|_| "Not UTF-8")?;

	let input = input.trim_start_matches('[').trim_end_matches(']');
	let input = Regex::new(", *")
		.expect("qed")
		.split(input)
		.collect::<Vec<_>>();
	let input = input
		.into_iter()
		.map(|x| x.parse::<u8>())
		.collect::<Result<Vec<_>, _>>()
		.map_err(|_| "Invalid byte array")?;

	let result: String = Hex::from(input).into();
	Ok(vec![result])
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			(
				"h2s",
				vec![Case {
					desc: "".to_string(),
					input: vec!["0x61626364"].into_iter().map(Into::into).collect(),
					output: vec!["abcd"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				}],
			),
			(
				"s2h",
				vec![Case {
					desc: "".to_string(),
					input: vec!["abcd"].into_iter().map(Into::into).collect(),
					output: vec!["0x61626364"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				}],
			),
			(
				"h2b",
				vec![Case {
					desc: "".to_string(),
					input: vec!["0x61626364"].into_iter().map(Into::into).collect(),
					output: vec!["abcd"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: false,
					since: "0.1.0".to_string(),
				}],
			),
			(
				"b2h",
				vec![Case {
					desc: "".to_string(),
					input: vec!["abcd"].into_iter().map(Into::into).collect(),
					output: vec!["0x61626364"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				}],
			),
			(
				"h2a",
				vec![Case {
					desc: "".to_string(),
					input: vec!["0x61626364"].into_iter().map(Into::into).collect(),
					output: vec!["[97, 98, 99, 100]"]
						.into_iter()
						.map(Into::into)
						.collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				}],
			),
			(
				"a2h",
				vec![Case {
					desc: "".to_string(),
					input: vec!["'[97, 98, 99, 100]'"]
						.into_iter()
						.map(Into::into)
						.collect(),
					output: vec!["0x61626364"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
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

	#[test]
	fn test_h2b() {
		let app = &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["h2b", "0x61626364"]);
		assert_eq!(h2b_inner(&matches), Ok(vec![0x61, 0x62, 0x63, 0x64]));
	}
}
