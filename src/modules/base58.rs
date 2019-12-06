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

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_h2b58() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["h2b58", "0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"]);
		assert_eq!(h2b58(&matches) , Ok(vec!["12dvBhvPEPniQmBmgvj4qpJEodT7P".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2b58", "0x41e1df5ec0fec39b5cf51d959118819ce5e64643df"]);
		assert_eq!(h2b58(&matches) , Ok(vec!["53yFvKp7qazhe1YV6rE5ESr1iofv6".to_string()]));
	}

	#[test]
	fn test_h2b58c() {
		let app =  &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["h2b58", "0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a"]);
		assert_eq!(h2b58c(&matches) , Ok(vec!["1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2b58", "0x41e1df5ec0fec39b5cf51d959118819ce5e64643df"]);
		assert_eq!(h2b58c(&matches) , Ok(vec!["TWZWdFSL6Kcn7hfAg6SHiGixUP5efEaBtW".to_string()]));

	}

	#[test]
	fn test_b582h() {
		let app =  &commands()[3].app;

		let matches = app.clone().get_matches_from(vec!["b582h", "3VNr6P"]);
		assert_eq!(b582h(&matches) , Ok(vec!["0x61626364".to_string()]));
	}

	#[test]
	fn test_b58c2h() {
		let app =  &commands()[3].app;

		let matches = app.clone().get_matches_from(vec!["b58c2h", "TGnEPsGyq5Hnb3CUJ56FSH6fQRtxgi3jYF"]);
		assert_eq!(b58c2h(&matches) , Ok(vec!["0x414ab577ca9adc8a3252dda006ea7221bded1cac17".to_string()]));
	}

}
