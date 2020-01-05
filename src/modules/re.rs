use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
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
		}
	]
}

fn re(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let pattern = matches.value_of("p").ok_or("Invalid pattern")?;

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

	#[test]
	fn test_re() {
		let app = &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["re", "-p", "a(.)c", "abc\nadc"]);
		assert_eq!(re(&matches), Ok(vec!["abc".to_string(), "    group#1: b".to_string(), "adc".to_string(), "    group#1: d".to_string()]));
	}
}
