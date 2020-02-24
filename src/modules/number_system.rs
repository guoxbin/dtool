use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Number 10/2/8/16 base conversion".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![Command {
		app: SubCommand::with_name("ns")
			.about("Number system")
			.arg(
				Arg::with_name("DECIMAL")
					.long("decimal")
					.short("d")
					.help("Output decimal result")
					.required(false),
			)
			.arg(
				Arg::with_name("BINARY")
					.long("binary")
					.short("b")
					.help("Output binary result")
					.required(false),
			)
			.arg(
				Arg::with_name("OCTAL")
					.long("octal")
					.short("o")
					.help("Output octal result")
					.required(false),
			)
			.arg(
				Arg::with_name("HEXADECIMAL")
					.long("hexadecimal")
					.short("x")
					.help("Output hexadecimal result")
					.required(false),
			)
			.arg(Arg::with_name("INPUT").required(false).index(1)),
		f: ns,
	}]
}

fn ns(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let (radix, number) = match input {
		_ if input.starts_with("0b") => (2, &input[2..]),
		_ if input.starts_with("0o") => (8, &input[2..]),
		_ if input.starts_with("0x") => (16, &input[2..]),
		_ => (10, &input[..]),
	};

	let number = u64::from_str_radix(number, radix).map_err(|_| "Invalid input")?;

	let mut results = Vec::new();

	if matches.is_present("DECIMAL") {
		results.push(format!("{}", number));
	}
	if matches.is_present("BINARY") {
		results.push(format!("0b{:b}", number));
	}
	if matches.is_present("OCTAL") {
		results.push(format!("0o{:o}", number));
	}
	if matches.is_present("HEXADECIMAL") {
		results.push(format!("0x{:x}", number));
	}
	if results.len() == 0 {
		results = vec![
			format!("{}", number),
			format!("0b{:b}", number),
			format!("0o{:o}", number),
			format!("0x{:x}", number),
		];
	}

	Ok(results)
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![(
			"ns",
			vec![
				Case {
					desc: "Input decimal".to_string(),
					input: vec!["256"].into_iter().map(Into::into).collect(),
					output: vec!["256", "0b100000000", "0o400", "0x100"]
						.into_iter()
						.map(Into::into)
						.collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
				Case {
					desc: "Input octal".to_string(),
					input: vec!["0o400"].into_iter().map(Into::into).collect(),
					output: vec!["256", "0b100000000", "0o400", "0x100"]
						.into_iter()
						.map(Into::into)
						.collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
				Case {
					desc: "Output decimal".to_string(),
					input: vec!["-d", "256"].into_iter().map(Into::into).collect(),
					output: vec!["256"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
				Case {
					desc: "Output binary".to_string(),
					input: vec!["-b", "256"].into_iter().map(Into::into).collect(),
					output: vec!["0b100000000"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
				Case {
					desc: "Output octal".to_string(),
					input: vec!["-o", "256"].into_iter().map(Into::into).collect(),
					output: vec!["0o400"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
				Case {
					desc: "Output hexadecimal".to_string(),
					input: vec!["-x", "256"].into_iter().map(Into::into).collect(),
					output: vec!["0x100"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.1.0".to_string(),
				},
			],
		)]
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
