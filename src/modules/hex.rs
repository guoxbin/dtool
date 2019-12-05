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

fn h2s(matches: &ArgMatches) {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};
	let input = input.trim_start_matches("0x");

	let result = hex::decode(input).map_err(|_| "Convert failed")
		.and_then(|x| String::from_utf8(x).map_err(|_| "Convert failed"));

	match result {
		Ok(result) => println!("{}", result),
		Err(e) => eprintln!("{}", e),
	}
}

fn s2h(matches: &ArgMatches) {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let result : Result<String, &str> = Ok(hex::encode(input)).map(|x| "0x".to_string() + &x);

	match result{
		Ok(result) => println!("{}", result),
		Err(e) => eprintln!("{}", e),
	}

}