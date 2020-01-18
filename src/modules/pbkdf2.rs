use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Module};
use ring::pbkdf2::{PBKDF2_HMAC_SHA1, PBKDF2_HMAC_SHA256, PBKDF2_HMAC_SHA384, PBKDF2_HMAC_SHA512, derive};
use std::num::NonZeroU32;
use crate::modules::base::Hex;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Pbkdf2".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("pbkdf2").about("Pbkdf2")
				.arg(
					Arg::with_name("ITERATIONS")
						.long("iterations")
						.short("i").help("Iterations")
						.takes_value(true)
						.default_value("1")
						.required(false))
				.arg(
					Arg::with_name("ALGORITHM")
						.long("algorithm")
						.short("a").help("Algorithm\nsha1: SHA-1\nsha2_256: SHA-2 256\nsha2_384: SHA-2 384\nsha2_512: SHA-2 512")
						.takes_value(true)
						.default_value("sha1")
						.required(false))
				.arg(
					Arg::with_name("SALT")
						.long("salt")
						.short("s").help("Salt (Hex)")
						.takes_value(true)
						.default_value("0x")
						.required(false))
				.arg(
					Arg::with_name("KEY_LENGTH")
						.long("key-length")
						.short("l").help("Key length")
						.takes_value(true)
						.default_value("128")
						.required(false))
				.arg(
					Arg::with_name("INPUT")
						.help("Secret (Hex)")
						.required(false)
						.index(1)),

			f: pbkdf2,
			cases: vec![
			],
		}
	]
}

fn pbkdf2(matches: &ArgMatches) -> Result<Vec<String>, String> {
	
	let algo = match matches.value_of("ALGORITHM") {
		Some("sha1") => PBKDF2_HMAC_SHA1,
		Some("sha2_256") => PBKDF2_HMAC_SHA256,
		Some("sha2_384") => PBKDF2_HMAC_SHA384,
		Some("sha2_512") => PBKDF2_HMAC_SHA512,
		_ => return Err("Invalid algorithm".to_string()),
	};

	let iterations = match matches.value_of("ITERATIONS") {
		Some(iterations) => iterations.parse::<u32>().map_err(|_|"Invalid Iterations".to_string()),
		_ => Err("Invalid Iterations".to_string()),
	}.and_then(|x| if x>0 {
		NonZeroU32::new(x).ok_or("Invalid Iterations".to_string())
	}else{
		Err("Invalid Iterations".to_string())
	})?;

	let salt : Vec<u8> = match matches.value_of("SALT") {
		Some(salt) => {
			salt.parse::<Hex>().map_err(|_|"Invalid salt".to_string())
		},
		_ => Err("Invalid salt".to_string()),
	}?.into();

	let key_length = match matches.value_of("KEY_LENGTH") {
		Some(key_length) => key_length.parse::<u32>().map_err(|_|"Invalid key length".to_string()),
		_ => Err("Invalid key length".to_string()),
	}.and_then(|x| if x>0 {
		Ok(x)
	}else{
		Err("Invalid key length".to_string())
	})?;

	let key_byte_length = if key_length % 8==0 {
		Ok(key_length / 8)
	}else{
		Err("Invalid key length".to_string())
	}?;

	let secret = base::input_string(matches)?;
	let secret : Vec<u8> = secret.parse::<Hex>().map_err(|_| "Invalid secret")?.into();

	let mut result = vec![0u8; key_byte_length as usize];

	derive(algo, iterations, &salt, &secret, &mut result);

	let result = Hex::from(result).into();

	Ok(vec![result])
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			("pbkdf2",
			 vec![
				 Case {
					 desc: "".to_string(),
					 input: vec!["-a", "sha2_256", "-s", "0x646566", "-i", "2", "-l", "256", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x51a30556d0d133d859d3f3da86f861b7b12546c4f9a193ebb374397467872514"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
			 ]),
		].into_iter().collect()
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
