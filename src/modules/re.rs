use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use regex::Regex;

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
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec!["-p", "a(.)c", "abc\nadc"].into_iter().map(Into::into).collect(),
					output: vec!["abc", "    group#1: b", "adc", "    group#1: d"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.4.0".to_string(),
				},
			],
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::base::test::test_commands;

	#[test]
	fn test_cases() {
		test_commands(&commands());
	}
}
