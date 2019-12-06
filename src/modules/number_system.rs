use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::Command;
use std::io;
use std::io::BufRead;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ns").about("Number system")
				.arg(
					Arg::with_name("d")
						.short("d").help("Output decimal result")
						.required(false))
				.arg(
					Arg::with_name("b")
						.short("b").help("Output binary result")
						.required(false))
				.arg(
					Arg::with_name("o")
						.short("o").help("Output octal result")
						.required(false))
				.arg(
					Arg::with_name("x")
						.short("x").help("Output hexadecimal result")
						.required(false))
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: ns,
		},
	]
}

fn ns(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let (radix, number) = match input {
		_ if input.starts_with("0b") => (2, &input[2..]),
		_ if input.starts_with("0o") => (8, &input[2..]),
		_ if input.starts_with("0x") => (16, &input[2..]),
		_ => (10, &input[..]),
	};

	let number = u64::from_str_radix(number, radix).map_err(|_| "Invalid input")?;

	let mut results = Vec::new();

	if matches.is_present("d") {
		results.push(format!("{}", number));
	}
	if matches.is_present("b") {
		results.push(format!("0b{:b}", number));
	}
	if matches.is_present("o") {
		results.push(format!("0o{:o}", number));
	}
	if matches.is_present("x") {
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
