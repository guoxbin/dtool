use self::Mode::CTR;
use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use yogcrypt::sm4;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "SM4 encrypt / decrypt".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("sm4_enc")
				.about("SM4 encrypt")
				.arg(
					Arg::with_name("INPUT")
						.help("Plain (Hex)")
						.required(false)
						.index(1),
				)
				.arg(
					Arg::with_name("MODE")
						.long("mode")
						.short("m")
						.help("Mode\nctr: CTR\n")
						.takes_value(true)
						.possible_values(&["ctr"])
						.required(true),
				)
				.arg(
					Arg::with_name("KEY")
						.long("key")
						.short("k")
						.help("Key (Hex)")
						.takes_value(true)
						.required(true),
				)
				.arg(
					Arg::with_name("IV")
						.long("iv")
						.short("i")
						.help("IV (Hex)")
						.takes_value(true)
						.required(false),
				),
			f: sm4_enc,
		},
		Command {
			app: SubCommand::with_name("sm4_dec")
				.about("SM4 decrypt")
				.arg(
					Arg::with_name("INPUT")
						.help("Cipher (Hex)")
						.required(false)
						.index(1),
				)
				.arg(
					Arg::with_name("MODE")
						.long("mode")
						.short("m")
						.help("Mode\nctr: CTR\n")
						.takes_value(true)
						.possible_values(&["ctr"])
						.required(true),
				)
				.arg(
					Arg::with_name("KEY")
						.long("key")
						.short("k")
						.help("Key (Hex)")
						.takes_value(true)
						.required(true),
				)
				.arg(
					Arg::with_name("IV")
						.long("iv")
						.short("i")
						.help("IV (Hex)")
						.takes_value(true)
						.required(false),
				),
			f: sm4_dec,
		},
	]
}

enum Mode {
	CTR { iv: Vec<u8> },
}

enum KeySize {
	KeySize128,
}

fn sm4_enc(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let (key_size, key, mode, input) = get_common_arg(matches)?;

	// cipher
	let result = match mode {
		CTR { iv } => sm4_enc_ctr(key_size, &key, &input, &iv),
	}?;
	let result = Hex::from(result).into();

	Ok(vec![result])
}

fn sm4_dec(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let (key_size, key, mode, input) = get_common_arg(matches)?;

	// plain
	let result = match mode {
		CTR { iv } => sm4_dec_ctr(key_size, &key, &input, &iv),
	}?;
	let result = Hex::from(result).into();

	Ok(vec![result])
}

fn get_common_arg(matches: &ArgMatches) -> Result<(KeySize, Vec<u8>, Mode, Vec<u8>), String> {
	let input = base::input_string(matches)?;

	// key and key_size
	let key = matches.value_of("KEY").ok_or("Invalid key".to_string())?;
	let key: Vec<u8> = key.parse::<Hex>().map_err(|_| "Invalid key")?.into();
	let key_size = match key.len() {
		16 => KeySize::KeySize128,
		_ => return Err("Invalid key size (should be 128)".to_string()),
	};

	let get_iv = || -> Result<Vec<u8>, String> {
		let iv = matches.value_of("IV").ok_or("Invalid IV".to_string())?;
		let iv: Vec<u8> = iv.parse::<Hex>().map_err(|_| "Invalid IV")?.into();
		Ok(iv)
	};

	// mode
	let mode = matches.value_of("MODE").ok_or("Invalid mode".to_string())?;
	let mode = match mode {
		"ctr" => CTR { iv: get_iv()? },
		_ => unreachable!(),
	};

	// input
	let input = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	Ok((key_size, key, mode, input))
}

const BLOCK_SIZE: usize = 16;

fn sm4_enc_ctr(key_size: KeySize, key: &[u8], input: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
	sm4_ctr_process(key_size, key, input, iv)
}

fn sm4_dec_ctr(key_size: KeySize, key: &[u8], input: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
	sm4_ctr_process(key_size, key, input, iv)
}

fn sm4_ctr_process(
	_key_size: KeySize,
	key: &[u8],
	input: &[u8],
	iv: &[u8],
) -> Result<Vec<u8>, String> {
	let mut buff = [0u8; BLOCK_SIZE];
	buff.copy_from_slice(iv);

	let block_count = input.len() / BLOCK_SIZE;
	let tail_len = input.len() % BLOCK_SIZE;

	let key = {
		let mut key_arr = [0u8; BLOCK_SIZE];
		key_arr.copy_from_slice(key);
		key_arr
	};

	let mut result = vec![0u8; input.len()];

	for i in 0..block_count {
		let enc = sm4::sm4_enc(&key, &buff);
		let ct = block_xor(&enc, &input[i * BLOCK_SIZE..i * BLOCK_SIZE + BLOCK_SIZE]);

		let to_write = &mut result[i * BLOCK_SIZE..i * BLOCK_SIZE + BLOCK_SIZE];
		to_write.copy_from_slice(&ct);
		block_add_one(&mut buff);
	}

	let enc = sm4::sm4_enc(&key, &buff);
	for i in 0..tail_len {
		let ii = block_count * 16 + i;
		let b = input[ii] ^ enc[i];
		result[ii] = b;
	}

	let result = result.to_vec();

	Ok(result)
}

fn block_xor(a: &[u8], b: &[u8]) -> [u8; BLOCK_SIZE] {
	let mut out: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
	for i in 0..BLOCK_SIZE {
		out[i] = a[i] ^ b[i];
	}
	out
}

fn block_add_one(a: &mut [u8]) {
	let mut t;
	let mut carry = 1;

	for i in 0..16 {
		t = i32::from(a[15 - i]) + carry;
		if t == 256 {
			t = 0;
			carry = 1;
		} else {
			carry = 0
		}
		a[15 - i] = t as u8;
	}
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			(
				"sm4_enc",
				vec![Case {
					desc: "CTR".to_string(),
					input: vec![
						"-k",
						"01010101010101010101010101010101",
						"-i",
						"03030303030303030303030303030303",
						"-m",
						"ctr",
						"0x616263",
					]
					.into_iter()
					.map(Into::into)
					.collect(),
					output: vec!["0x8cd7ea"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.6.0".to_string(),
				}],
			),
			(
				"sm4_dec",
				vec![Case {
					desc: "CTR".to_string(),
					input: vec![
						"-k",
						"01010101010101010101010101010101",
						"-i",
						"03030303030303030303030303030303",
						"-m",
						"ctr",
						"0x8cd7ea",
					]
					.into_iter()
					.map(Into::into)
					.collect(),
					output: vec!["0x616263"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				}],
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
