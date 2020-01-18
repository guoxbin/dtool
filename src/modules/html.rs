use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Module};
use escaper;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "HTML entity encode / decode".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("he").about("HTML entity encode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: he,
			cases: vec![],
		},
		Command {
			app: SubCommand::with_name("hd").about("HTML entity decode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: hd,
			cases: vec![],
		}
	]
}

fn he(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let result = escaper::encode_minimal(&input);

	Ok(vec![result])
}

fn hd(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let result = escaper::decode_html(&input).map_err(|_| "Decode failed")?;

	Ok(vec![result])
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			("he",
			 vec![
				 Case {
					 desc: "".to_string(),
					 input: vec!["'<b>'"].into_iter().map(Into::into).collect(),
					 output: vec!["&lt;b&gt;"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.4.0".to_string(),
				 },
			 ]),
			("hd",
			 vec![
				 Case {
					 desc: "".to_string(),
					 input: vec!["'&lt;b&gt;'"].into_iter().map(Into::into).collect(),
					 output: vec!["<b>"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.4.0".to_string(),
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
