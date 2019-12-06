use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::Command;
use std::io;
use std::io::BufRead;
use hex;
use bs58;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2b58").about("Convert hex to base58").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2b58,
		},
		Command {
			app: SubCommand::with_name("h2b58c").about("Convert hex to base58 check").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2b58c,
		},
		Command {
			app: SubCommand::with_name("b582h").about("Convert base58 to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: b582h,
		},
		Command {
			app: SubCommand::with_name("b58c2h").about("Convert base58 check to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: b58c2h,
		},
	]
}

fn h2b58(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let result = bs58::encode(input).into_string();

	Ok(vec![result])
}

fn h2b58c(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let result = bs58::encode(input).with_check().into_string();

	Ok(vec![result])
}

fn b582h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let input = bs58::decode(&input).into_vec().map_err(|_| "Convert failed")?;
	let result = hex::encode(input);
	let result = format!("0x{}", result);

	Ok(vec![result])
}

fn b58c2h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let input = bs58::decode(&input).with_check(None).into_vec().map_err(|_| "Convert failed")?;
	let result = hex::encode(input);
	let result = format!("0x{}", result);

	Ok(vec![result])
}
