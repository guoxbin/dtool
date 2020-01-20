use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Module};
use regex::Regex;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Regex match".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("re").about("Regex match")
				.arg(
					Arg::with_name("PATTERN")
						.long("pattern")
						.short("p").help("Regex pattern")
						.takes_value(true)
						.required(true))
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: re,
		}
	]
}

fn re(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let pattern = matches.value_of("PATTERN").ok_or("Invalid pattern")?;

	let pattern = Regex::new(pattern).map_err(|_| "Invalid pattern")?;

	let mut result = vec![];

	for (_i, c) in pattern.captures_iter(&input).enumerate() {
		for (j, x) in c.iter().enumerate() {
			if j == 0 {
				result.push(format!("{}", x.unwrap().as_str()));
			} else {
				result.push(format!("    group#{}: {}", j, x.unwrap().as_str()));
			}
		}
	}

	Ok(result)
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			("re",
			 vec![
				 Case {
					 desc: "".to_string(),
					 input: vec!["-p", "'a(.)c'", "abcadc"].into_iter().map(Into::into).collect(),
					 output: vec!["abc", "    group#1: b", "adc", "    group#1: d"].into_iter().map(Into::into).collect(),
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
