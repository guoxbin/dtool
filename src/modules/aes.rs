use self::Mode::{CBC, CTR, ECB};
use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use crypto::aes::{cbc_decryptor, cbc_encryptor, ctr, ecb_decryptor, ecb_encryptor, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor};

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "AES encrypt / decrypt".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("aes_enc")
				.about("AES encrypt")
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
						.help("Mode\necb: ECB\ncbc: CBC\nctr: CTR\n")
						.takes_value(true)
						.possible_values(&["ecb", "cbc", "ctr"])
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
			f: aes_enc,
		},
		Command {
			app: SubCommand::with_name("aes_dec")
				.about("AES decrypt")
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
						.help("Mode\necb: ECB\ncbc: CBC\nctr: CTR\n")
						.takes_value(true)
						.possible_values(&["ecb", "cbc", "ctr"])
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
			f: aes_dec,
		},
	]
}

enum Mode {
	ECB,
	CBC { iv: Vec<u8> },
	CTR { iv: Vec<u8> },
}

fn aes_enc(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let (key_size, key, mode, input) = get_common_arg(matches)?;

	// cipher
	let result = match mode {
		ECB => aes_enc_ecb(key_size, &key, &input),
		CBC { iv } => aes_enc_cbc(key_size, &key, &input, &iv),
		CTR { iv } => aes_enc_ctr(key_size, &key, &input, &iv),
	}?;
	let result = Hex::from(result).into();

	Ok(vec![result])
}

fn aes_dec(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let (key_size, key, mode, input) = get_common_arg(matches)?;

	// plain
	let result = match mode {
		ECB => aes_dec_ecb(key_size, &key, &input),
		CBC { iv } => aes_dec_cbc(key_size, &key, &input, &iv),
		CTR { iv } => aes_dec_ctr(key_size, &key, &input, &iv),
	}?;
	let result = Hex::from(result).into();

	Ok(vec![result])
}

fn get_common_arg(matches: &ArgMatches) -> Result<(KeySize, Vec<u8>, Mode, Vec<u8>), String> {
	let input = base::input_string(matches)?;

	// key and key_size
	let key = matches
		.value_of("KEY")
		.ok_or_else(|| "Invalid key".to_string())?;
	let key: Vec<u8> = key.parse::<Hex>().map_err(|_| "Invalid key")?.into();
	let key_size = match key.len() {
		16 => KeySize::KeySize128,
		24 => KeySize::KeySize192,
		32 => KeySize::KeySize256,
		_ => return Err("Invalid key size (should be 128/192/256)".to_string()),
	};

	let get_iv = || -> Result<Vec<u8>, String> {
		let iv = matches
			.value_of("IV")
			.ok_or_else(|| "Invalid IV".to_string())?;
		let iv: Vec<u8> = iv.parse::<Hex>().map_err(|_| "Invalid IV")?.into();
		Ok(iv)
	};

	// mode
	let mode = matches
		.value_of("MODE")
		.ok_or_else(|| "Invalid mode".to_string())?;
	let mode = match mode {
		"ecb" => ECB,
		"cbc" => CBC { iv: get_iv()? },
		"ctr" => CTR { iv: get_iv()? },
		_ => unreachable!(),
	};

	// input
	let input = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	Ok((key_size, key, mode, input))
}

fn aes_enc_ecb(key_size: KeySize, key: &[u8], input: &[u8]) -> Result<Vec<u8>, String> {
	let mut a = ecb_encryptor(key_size, key, PkcsPadding);
	let cipher_len = cipher_length(input.len());
	let mut result = vec![0u8; cipher_len];
	a.encrypt(
		&mut RefReadBuffer::new(&input),
		&mut RefWriteBuffer::new(&mut result),
		true,
	)
	.map_err(|_| "Enc failed")?;
	Ok(result)
}

fn aes_enc_cbc(key_size: KeySize, key: &[u8], input: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
	let mut a = cbc_encryptor(key_size, key, iv, PkcsPadding);
	let cipher_len = cipher_length(input.len());
	let mut result = vec![0u8; cipher_len];
	a.encrypt(
		&mut RefReadBuffer::new(&input),
		&mut RefWriteBuffer::new(&mut result),
		true,
	)
	.map_err(|_| "Enc failed")?;
	Ok(result)
}

fn aes_enc_ctr(key_size: KeySize, key: &[u8], input: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
	let mut a = ctr(key_size, key, iv);
	let mut result = vec![0u8; input.len()];
	a.encrypt(
		&mut RefReadBuffer::new(&input),
		&mut RefWriteBuffer::new(&mut result),
		true,
	)
	.map_err(|_| "Enc failed")?;
	Ok(result)
}

fn aes_dec_ecb(key_size: KeySize, key: &[u8], input: &[u8]) -> Result<Vec<u8>, String> {
	let mut a = ecb_decryptor(key_size, key, PkcsPadding);
	let mut result = vec![0u8; input.len()];
	let mut buffer = RefWriteBuffer::new(&mut result);
	a.decrypt(&mut RefReadBuffer::new(&input), &mut buffer, true)
		.map_err(|_| "Dec failed")?;
	let len = buffer.capacity() - buffer.remaining();
	let mut result = result.clone();
	result.truncate(len);
	Ok(result)
}

fn aes_dec_cbc(key_size: KeySize, key: &[u8], input: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
	let mut a = cbc_decryptor(key_size, key, iv, PkcsPadding);
	let mut result = vec![0u8; input.len()];
	let mut buffer = RefWriteBuffer::new(&mut result);
	a.decrypt(&mut RefReadBuffer::new(&input), &mut buffer, true)
		.map_err(|_| "Dec failed")?;
	let len = buffer.capacity() - buffer.remaining();
	let mut result = result.clone();
	result.truncate(len);
	Ok(result)
}

fn aes_dec_ctr(key_size: KeySize, key: &[u8], input: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
	let mut a = ctr(key_size, key, iv);
	let mut result = vec![0u8; input.len()];
	let mut buffer = RefWriteBuffer::new(&mut result);
	a.decrypt(&mut RefReadBuffer::new(&input), &mut buffer, true)
		.map_err(|_| "Dec failed")?;
	Ok(result)
}

const BLOCK_SIZE: usize = 16;

fn cipher_length(input_len: usize) -> usize {
	((input_len / BLOCK_SIZE) + 1) * BLOCK_SIZE
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			(
				"aes_enc",
				vec![
					Case {
						desc: "KeySize 128 ECB".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-m",
							"ecb",
							"0x616263646162636461626364616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0xe89c98329f3e8b6da3e714fbba2be6d1"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 ECB".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-m",
							"ecb",
							"0x61626364616263646162636461626364",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec![
							"0xd7f480b25ee881f4b14d9893e6d76e7d68434d37d7f69f0e03b75bb15bc94b6c",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 ECB".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x616263646162636461626364616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x88fe17738e31914c9166f9b101d1b028"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 ECB".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x61626364616263646162636461626364",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec![
							"0x163d5ef52036845917bd95ef5f4adc1bc5e91d8668d614923d5e38a3f5fb895d",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 ECB".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x616263646162636461626364616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x3e6bcc9d26c494b1c6971316020acd3a"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 ECB".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x61626364616263646162636461626364",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec![
							"0x5bd66672cb9f17f96a2684a815efa024cce1a41155667587123071beb30ed5c8",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 CBC".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x616263646162636461626364616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x350678b99c37ab5f68f560551e960572"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 CBC".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x61626364616263646162636461626364",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec![
							"0x292d1fba6bf1c22fa8487591b71ac04415a5e65a5a17ada18718df37025abd1f",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 CBC".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x616263646162636461626364616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0xbbc8ff4de1a197e67a5f8f4d7a35f9a0"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 CBC".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x61626364616263646162636461626364",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec![
							"0xee6e039c879dcd303599f26f992ef00f0ee4d0e9aee9d65c751be72510368a51",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 CBC".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x616263646162636461626364616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x3309a7511f007e993676a90a06391d28"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 CBC".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x61626364616263646162636461626364",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec![
							"0xe81f4279192c2eca7a1bfcf171c352bf512f1379831e41c71a83cdda50b84205",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 CTR".to_string(),
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
						output: vec!["0x075e64"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 CTR".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"ctr",
							"0x616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0xbad37a"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 CTR".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"ctr",
							"0x616263",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x9e5062"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
				],
			),
			(
				"aes_dec",
				vec![
					Case {
						desc: "KeySize 128 ECB".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-m",
							"ecb",
							"0xe89c98329f3e8b6da3e714fbba2be6d1",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263646162636461626364616263"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 ECB".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-m",
							"ecb",
							"0x9628c4db5e8f782545876a9e1f676bdb9feda21b5557c81308f95a50e1a16232",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x6162636461626364616263646162630000"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 ECB".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-m",
							"ecb",
							"0xd7f480b25ee881f4b14d9893e6d76e7d68434d37d7f69f0e03b75bb15bc94b6c",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x61626364616263646162636461626364"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 ECB".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x88fe17738e31914c9166f9b101d1b028",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263646162636461626364616263"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 ECB".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x163d5ef52036845917bd95ef5f4adc1bc5e91d8668d614923d5e38a3f5fb895d",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x61626364616263646162636461626364"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 ECB".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x3e6bcc9d26c494b1c6971316020acd3a",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263646162636461626364616263"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 ECB".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-m",
							"ecb",
							"0x5bd66672cb9f17f96a2684a815efa024cce1a41155667587123071beb30ed5c8",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x61626364616263646162636461626364"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 CBC".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x350678b99c37ab5f68f560551e960572",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263646162636461626364616263"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 CBC".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x292d1fba6bf1c22fa8487591b71ac04415a5e65a5a17ada18718df37025abd1f",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x61626364616263646162636461626364"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 CBC".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0xbbc8ff4de1a197e67a5f8f4d7a35f9a0",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263646162636461626364616263"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 CBC".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0xee6e039c879dcd303599f26f992ef00f0ee4d0e9aee9d65c751be72510368a51",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x61626364616263646162636461626364"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 CBC".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0x3309a7511f007e993676a90a06391d28",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263646162636461626364616263"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 CBC".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"cbc",
							"0xe81f4279192c2eca7a1bfcf171c352bf512f1379831e41c71a83cdda50b84205",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x61626364616263646162636461626364"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: false,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 128 CTR".to_string(),
						input: vec![
							"-k",
							"01010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"ctr",
							"0x075e64",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 192 CTR".to_string(),
						input: vec![
							"-k",
							"010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"ctr",
							"0xbad37a",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
					},
					Case {
						desc: "KeySize 256 CTR".to_string(),
						input: vec![
							"-k",
							"0101010101010101010101010101010101010101010101010101010101010101",
							"-i",
							"03030303030303030303030303030303",
							"-m",
							"ctr",
							"0x9e5062",
						]
						.into_iter()
						.map(Into::into)
						.collect(),
						output: vec!["0x616263"].into_iter().map(Into::into).collect(),
						is_example: true,
						is_test: true,
						since: "0.6.0".to_string(),
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
