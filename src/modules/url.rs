use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::Command;
use std::io;
use std::io::BufRead;
use urlencoding;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ue").about("URL encode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ue,
		},
		Command {
			app: SubCommand::with_name("ud").about("URL decode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ud,
		}
	]
}

fn ue(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let result = urlencoding::encode(&input);

	Ok(vec![result])
}

fn ud(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let result = urlencoding::decode(&input).map_err(|_| "Decode failed")?;

	Ok(vec![result])
}