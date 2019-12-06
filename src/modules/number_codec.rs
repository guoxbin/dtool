use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::Command;
use std::io;
use std::io::BufRead;
use hex;
use parity_codec::{Compact, Encode, Decode};

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ne").about("Number encode")
				.arg(
				Arg::with_name("t")
					.short("t").help("Number type: u8, u16, u32, u64, u128, c(Compact)")
					.takes_value(true)
					.required(true))
				.arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ne,
		},
		Command {
			app: SubCommand::with_name("nd").about("Number decode")
				.arg(
					Arg::with_name("t")
						.short("t").help("Number type: u8, u16, u32, u64, u128, c(Compact)")
						.takes_value(true)
						.required(true))
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: nd,
		},
	]
}

fn ne(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let t = matches.value_of("t").ok_or("Invalid number type")?;

	let result = match t{
		"u8" => {
			let input = input.parse::<u8>().map_err(|_|"Invalid input")?;
			vec![input]
		},
		"u16" => {
			let input = input.parse::<u16>().map_err(|_|"Invalid input")?;
			input.encode()
		},
		"u32" => {
			let input = input.parse::<u32>().map_err(|_|"Invalid input")?;
			input.encode()
		},
		"u64" => {
			let input = input.parse::<u64>().map_err(|_|"Invalid input")?;
			input.encode()
		},
		"u128" => {
			let input = input.parse::<u128>().map_err(|_|"Invalid input")?;
			input.encode()
		},
		"c" => {
			let input = Compact(input.parse::<u128>().map_err(|_|"Invalid input")?);
			input.encode()
		},
		_ => return Err("Invalid input".to_string()),
	};

	let result = hex::encode(result);

	let result = format!("0x{}", result);

	Ok(vec![result])
}

fn nd(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let t = matches.value_of("t").ok_or("Invalid number type")?;

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_|"Invalid input")?;

	let mut input = &input[..];

	let result = match t{
		"u8" => {
			let input : u8 = if input.len()>0 { input[0] } else { return Err("Invalid input".to_string()) };
			format!("{}", input)
		},
		"u16" => {
			let input : u16 = Decode::decode(&mut input).ok_or("Invalid input")?;
			format!("{}", input)
		},
		"u32" => {
			let input : u32 = Decode::decode(&mut input).ok_or("Invalid input")?;
			format!("{}", input)
		},
		"u64" => {
			let input : u64 = Decode::decode(&mut input).ok_or("Invalid input")?;
			format!("{}", input)
		},
		"u128" => {
			let input : u128 = Decode::decode(&mut input).ok_or("Invalid input")?;
			format!("{}", input)
		},
		"c" => {
			let input : Compact<u128> = Decode::decode(&mut input).ok_or("Invalid input")?;
			format!("{}", input.0)
		},
		_ => return Err("Invalid input".to_string()),
	};

	Ok(vec![result])
}
