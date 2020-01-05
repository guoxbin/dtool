use clap::ArgMatches;
use std::io;
use std::io::{BufRead, Read};

pub fn input_string(matches: &ArgMatches) -> Result<String, String> {
	match matches.value_of("INPUT") {
		Some(input) => Ok(input.to_string()),
		None => io::stdin().lock().lines().collect::<Result<Vec<String>, io::Error>>().map(|x|x.join("\n")).map_err(|_| "Invalid input".to_string()),
	}
}

pub fn input_bytes(matches: &ArgMatches) -> Result<Vec<u8>, String> {
	match matches.value_of("INPUT") {
		Some(input) => Ok(input.bytes().collect::<Vec<u8>>()),
		None => io::stdin().bytes().collect::<Result<Vec<u8>, io::Error>>().map_err(|_| "Invalid input".to_string()),
	}
}

#[cfg(test)]
pub mod test {
	use crate::modules::Command;

	pub fn test_commands(commands: &Vec<Command>) {
		for command in commands {
			let app = &command.app;
			let cases = &command.cases;
			let f = &command.f.clone();
			for (_i, case) in cases.iter().enumerate() {
				if case.is_test {
					let mut input = vec![app.get_name().to_string()];
					input.append(&mut case.input.clone());
					let expected_output = Ok((&case.output).clone());
					let matches = app.clone().get_matches_from(input.clone());
					let output = f(&matches);
					assert_eq!(output, expected_output, "Test: {}", input.join(" "));
				}
			}
		}
	}
}