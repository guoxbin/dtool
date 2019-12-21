use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
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

	let input = base::input_string(matches)?;

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
	let input = base::input_string(matches)?;

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

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_ne() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["ne", "-tu8", "1"]);
		assert_eq!(ne(&matches) , Ok(vec!["0x01".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ne", "-tu16", "1"]);
		assert_eq!(ne(&matches) , Ok(vec!["0x0100".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ne", "-tu32", "1"]);
		assert_eq!(ne(&matches) , Ok(vec!["0x01000000".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ne", "-tu64", "1"]);
		assert_eq!(ne(&matches) , Ok(vec!["0x0100000000000000".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ne", "-tu128", "1"]);
		assert_eq!(ne(&matches) , Ok(vec!["0x01000000000000000000000000000000".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ne", "-tc", "6"]);
		assert_eq!(ne(&matches) , Ok(vec!["0x18".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ne", "-tc", "251"]);
		assert_eq!(ne(&matches) , Ok(vec!["0xed03".to_string()]));
	}

	#[test]
	fn test_nd() {
		let app =  &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["nd", "-tu8", "0x01"]);
		assert_eq!(nd(&matches) , Ok(vec!["1".to_string()]));

		let matches = app.clone().get_matches_from(vec!["nd", "-tu16", "0x0100"]);
		assert_eq!(nd(&matches) , Ok(vec!["1".to_string()]));

		let matches = app.clone().get_matches_from(vec!["nd", "-tu32", "0x01000000"]);
		assert_eq!(nd(&matches) , Ok(vec!["1".to_string()]));

		let matches = app.clone().get_matches_from(vec!["nd", "-tu64", "0x0100000000000000"]);
		assert_eq!(nd(&matches) , Ok(vec!["1".to_string()]));

		let matches = app.clone().get_matches_from(vec!["nd", "-tu128", "0x01000000000000000000000000000000"]);
		assert_eq!(nd(&matches) , Ok(vec!["1".to_string()]));

		let matches = app.clone().get_matches_from(vec!["nd", "-tc", "0x18"]);
		assert_eq!(nd(&matches) , Ok(vec!["6".to_string()]));

		let matches = app.clone().get_matches_from(vec!["nd", "-tc", "0xed03"]);
		assert_eq!(nd(&matches) , Ok(vec!["251".to_string()]));

	}

}
