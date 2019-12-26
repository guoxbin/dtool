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
