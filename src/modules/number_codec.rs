use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use parity_codec::{Compact, Decode, Encode};

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Number codec".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ne")
				.about("Number encode")
				.arg(
					Arg::with_name("TYPE")
						.long("type")
						.short("t")
						.help("Number type\nu8\nu16\nu32\nu64\nu128\nc: Compact")
						.takes_value(true)
						.required(true),
				)
				.arg(
					Arg::with_name("ENDIAN")
						.long("endian")
						.short("e")
						.help("Endian\nlittle\nbig")
						.default_value("little")
						.takes_value(true)
						.required(false),
				)
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: ne,
		},
		Command {
			app: SubCommand::with_name("nd")
				.about("Number decode")
				.arg(
					Arg::with_name("TYPE")
						.long("type")
						.short("t")
						.help("Number type: u8, u16, u32, u64, u128, c(Compact)")
						.takes_value(true)
						.required(true),
				)
				.arg(
					Arg::with_name("ENDIAN")
						.long("endian")
						.short("e")
						.help("Endian\nlittle\nbig")
						.default_value("little")
						.takes_value(true)
						.required(false),
				)
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: nd,
		},
	]
}

fn ne(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let t = matches.value_of("TYPE").ok_or("Invalid number type")?;
	let e = matches.value_of("ENDIAN").ok_or("Invalid endian")?;
	let big = match e {
		"big" => true,
		"little" => false,
		_ => return Err("Invalid endian".to_string()),
	};

	let result = match t {
		"u8" => {
			let input = input.parse::<u8>().map_err(|_| "Invalid input")?;
			let input = if big { input.to_be() } else { input };
			vec![input]
		}
		"u16" => {
			let input = input.parse::<u16>().map_err(|_| "Invalid input")?;
			let input = if big { input.to_be() } else { input };
			input.encode()
		}
		"u32" => {
			let input = input.parse::<u32>().map_err(|_| "Invalid input")?;
			let input = if big { input.to_be() } else { input };
			input.encode()
		}
		"u64" => {
			let input = input.parse::<u64>().map_err(|_| "Invalid input")?;
			let input = if big { input.to_be() } else { input };
			input.encode()
		}
		"u128" => {
			let input = input.parse::<u128>().map_err(|_| "Invalid input")?;
			let input = if big { input.to_be() } else { input };
			input.encode()
		}
		"c" => {
			let input = Compact(input.parse::<u128>().map_err(|_| "Invalid input")?);
			input.encode()
		}
		_ => return Err("Invalid input".to_string()),
	};

	let result = Hex::from(result).into();

	Ok(vec![result])
}

fn nd(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let t = matches.value_of("TYPE").ok_or("Invalid number type")?;

	let e = matches.value_of("ENDIAN").ok_or("Invalid endian")?;
	let big = match e {
		"big" => true,
		"little" => false,
		_ => return Err("Invalid endian".to_string()),
	};

	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	let mut input = &input[..];

	let result = match t {
		"u8" => {
			let input: u8 = if input.len() > 0 {
				input[0]
			} else {
				return Err("Invalid input".to_string());
			};
			let input = if big { input.to_be() } else { input };
			format!("{}", input)
		}
		"u16" => {
			let input: u16 = Decode::decode(&mut input).ok_or("Invalid input")?;
			let input = if big { input.to_be() } else { input };
			format!("{}", input)
		}
		"u32" => {
			let input: u32 = Decode::decode(&mut input).ok_or("Invalid input")?;
			let input = if big { input.to_be() } else { input };
			format!("{}", input)
		}
		"u64" => {
			let input: u64 = Decode::decode(&mut input).ok_or("Invalid input")?;
			let input = if big { input.to_be() } else { input };
			format!("{}", input)
		}
		"u128" => {
			let input: u128 = Decode::decode(&mut input).ok_or("Invalid input")?;
			let input = if big { input.to_be() } else { input };
			format!("{}", input)
		}
		"c" => {
			let input: Compact<u128> = Decode::decode(&mut input).ok_or("Invalid input")?;
			format!("{}", input.0)
		}
		_ => return Err("Invalid input".to_string()),
	};

	Ok(vec![result])
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			(
				"ne",
				vec![
					Case {
						desc: "u8".to_string(),
						input: vec!["-tu8", "1"].into_iter().map(Into::into).collect(),
						output: vec!["0x01"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u16".to_string(),
						input: vec!["-tu16", "1"].into_iter().map(Into::into).collect(),
						output: vec!["0x0100"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u16".to_string(),
						input: vec!["-tu16", "-ebig", "1"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["0x0001"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u32".to_string(),
						input: vec!["-tu32", "1"].into_iter().map(Into::into).collect(),
						output: vec!["0x01000000"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u32".to_string(),
						input: vec!["-tu32", "-ebig", "1"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["0x00000001"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u64".to_string(),
						input: vec!["-tu64", "1"].into_iter().map(Into::into).collect(),
						output: vec!["0x0100000000000000"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u128".to_string(),
						input: vec!["-tu128", "1"].into_iter().map(Into::into).collect(),
						output: vec!["0x01000000000000000000000000000000"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "Compact".to_string(),
						input: vec!["-tc", "6"].into_iter().map(Into::into).collect(),
						output: vec!["0x18"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "Compact".to_string(),
						input: vec!["-tc", "251"].into_iter().map(Into::into).collect(),
						output: vec!["0xed03"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
				],
			),
			(
				"nd",
				vec![
					Case {
						desc: "u8".to_string(),
						input: vec!["-tu8", "0x01"].into_iter().map(Into::into).collect(),
						output: vec!["1"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u16".to_string(),
						input: vec!["-tu16", "0x0100"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["1"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u16".to_string(),
						input: vec!["-tu16", "-ebig", "0x0001"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["1"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u32".to_string(),
						input: vec!["-tu32", "0x01000000"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["1"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u32".to_string(),
						input: vec!["-tu32", "-ebig", "0x00000001"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["1"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u64".to_string(),
						input: vec!["-tu64", "0x0100000000000000"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["1"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "u128".to_string(),
						input: vec!["-tu128", "0x01000000000000000000000000000000"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["1"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "Compact".to_string(),
						input: vec!["-tc", "0x18"].into_iter().map(Into::into).collect(),
						output: vec!["6"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
					Case {
						desc: "Compact".to_string(),
						input: vec!["-tc", "0xed03"].into_iter().map(Into::into).collect(),
						output: vec!["251"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.1.0".to_string(),
					},
				],
			),
		]
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
