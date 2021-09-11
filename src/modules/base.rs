use clap::ArgMatches;
use std::io;
use std::io::{BufRead, Read};
use std::str::FromStr;

pub fn input_string(matches: &ArgMatches) -> Result<String, String> {
	match matches.value_of("INPUT") {
		Some(input) => Ok(input.to_string()),
		None => io::stdin()
			.lock()
			.lines()
			.collect::<Result<Vec<String>, io::Error>>()
			.map(|x| x.join("\n"))
			.map_err(|_| "Invalid input".to_string()),
	}
}

pub fn input_bytes(matches: &ArgMatches) -> Result<Vec<u8>, String> {
	match matches.value_of("INPUT") {
		Some(input) => Ok(input.bytes().collect::<Vec<u8>>()),
		None => io::stdin()
			.bytes()
			.collect::<Result<Vec<u8>, io::Error>>()
			.map_err(|_| "Invalid input".to_string()),
	}
}

pub struct Hex(Vec<u8>);

impl FromStr for Hex {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = hex::decode(s.trim_start_matches("0x")).map_err(|_| "Invalid hex".to_string())?;
		Ok(Self(s))
	}
}

impl From<Vec<u8>> for Hex {
	fn from(f: Vec<u8>) -> Self {
		Self(f)
	}
}

impl From<Hex> for String {
	fn from(v: Hex) -> Self {
		format!("0x{}", hex::encode(v.0))
	}
}

impl From<Hex> for Vec<u8> {
	fn from(v: Hex) -> Self {
		v.0
	}
}

#[cfg(test)]
pub mod test {
	use crate::modules::Module;

	pub fn test_module(module: Module) {
		let commands = module.commands;
		let cases = (module.get_cases)();
		for command in commands {
			let app = &command.app;
			let cases = cases.get(app.get_name());

			assert!(cases.is_some(), "{} should have cases", app.get_name());
			if let Some(cases) = cases {
				assert!(cases.len() > 0, "{} should have cases", app.get_name());

				let f = &command.f.clone();
				for case in cases {
					if case.is_test {
						let mut ori_input = case
							.input
							.clone()
							.into_iter()
							.map(|x| {
								let x = x.trim_start_matches("'");
								let x = x.trim_end_matches("'");
								x.to_string()
							})
							.collect();
						let mut input = vec![app.get_name().to_string()];
						input.append(&mut ori_input);
						let expected_output = Ok((&case.output).clone());
						let matches = app.clone().get_matches_from(input.clone());
						let output = f(&matches);
						assert_eq!(output, expected_output, "Test: {}", input.join(" "));
					}
				}
			}
		}
	}
}
