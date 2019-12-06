use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::Command;
use std::io;
use std::io::BufRead;
use hex;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2s").about("Convert hex to string").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2s,
		},
		Command {
			app: SubCommand::with_name("s2h").about("Convert string to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: s2h,
		}
	]
}

fn h2s(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};
	let input = input.trim_start_matches("0x");

	let result = hex::decode(input).map_err(|_| "Convert failed")?;
	let result = String::from_utf8(result).map_err(|_| "Convert failed")?;

	Ok(vec![result])
}

fn s2h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let result = hex::encode(input);
	let result = "0x".to_string() + &result;

	Ok(vec![result])
}