use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use crc::crc32;

use lazy_static::lazy_static;
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY};
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};
use std::collections::HashMap;
use yogcrypt::sm3::sm3_enc;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Hash (MD5, SHA-1, SHA-2, SHA-3, RIPEMD, CRC, Blake2b, SM3, Twox)".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

struct Algorithm {
	name: &'static str,
	help: &'static str,
	f: AlgorithmF,
}

enum AlgorithmF {
	Normal(fn(data: Vec<u8>) -> Result<Vec<u8>, String>),
	WithKey(fn(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, String>),
	WithSeed(fn(data: Vec<u8>, seed: u64) -> Result<Vec<u8>, String>),
}

lazy_static! {
	static ref RAW_ALGORITHMS: Vec<Algorithm> = vec![
		Algorithm {
			name: "md5",
			help: "MD5",
			f: AlgorithmF::Normal(md5),
		},
		Algorithm {
			name: "sha1",
			help: "SHA-1",
			f: AlgorithmF::Normal(sha1),
		},
		Algorithm {
			name: "sha2_224",
			help: "SHA-2 224",
			f: AlgorithmF::Normal(sha2_224),
		},
		Algorithm {
			name: "sha2_256",
			help: "SHA-2 256",
			f: AlgorithmF::Normal(sha2_256),
		},
		Algorithm {
			name: "sha2_384",
			help: "SHA-2 384",
			f: AlgorithmF::Normal(sha2_384),
		},
		Algorithm {
			name: "sha2_512",
			help: "SHA-2 512",
			f: AlgorithmF::Normal(sha2_512),
		},
		Algorithm {
			name: "sha2_512_224",
			help: "SHA-2 512 truncate 224",
			f: AlgorithmF::Normal(sha2_512_224),
		},
		Algorithm {
			name: "sha2_512_256",
			help: "SHA-2 512 truncate 256",
			f: AlgorithmF::Normal(sha2_512_256),
		},
		Algorithm {
			name: "sha3_224",
			help: "SHA-3 224",
			f: AlgorithmF::Normal(sha3_224),
		},
		Algorithm {
			name: "sha3_256",
			help: "SHA-3 256",
			f: AlgorithmF::Normal(sha3_256),
		},
		Algorithm {
			name: "sha3_384",
			help: "SHA-3 384",
			f: AlgorithmF::Normal(sha3_384),
		},
		Algorithm {
			name: "sha3_512",
			help: "SHA-3 512",
			f: AlgorithmF::Normal(sha3_512),
		},
		Algorithm {
			name: "sha3_k_224",
			help: "SHA-3 keccak 224",
			f: AlgorithmF::Normal(sha3_k_224),
		},
		Algorithm {
			name: "sha3_k_256",
			help: "SHA-3 keccak 256",
			f: AlgorithmF::Normal(sha3_k_256),
		},
		Algorithm {
			name: "sha3_k_384",
			help: "SHA-3 keccak 384",
			f: AlgorithmF::Normal(sha3_k_384),
		},
		Algorithm {
			name: "sha3_k_512",
			help: "SHA-3 keccak 512",
			f: AlgorithmF::Normal(sha3_k_512),
		},
		Algorithm {
			name: "ripemd_160",
			help: "RIPEMD-160",
			f: AlgorithmF::Normal(ripemd_160),
		},
		Algorithm {
			name: "crc_32",
			help: "CRC32",
			f: AlgorithmF::Normal(crc_32),
		},
		Algorithm {
			name: "blake2b_160",
			help: "Blake2b 160",
			f: AlgorithmF::WithKey(blake2b_160),
		},
		Algorithm {
			name: "blake2b_256",
			help: "Blake2b 256",
			f: AlgorithmF::WithKey(blake2b_256),
		},
		Algorithm {
			name: "blake2b_384",
			help: "Blake2b 384",
			f: AlgorithmF::WithKey(blake2b_384),
		},
		Algorithm {
			name: "blake2b_512",
			help: "Blake2b 512",
			f: AlgorithmF::WithKey(blake2b_512),
		},
		Algorithm {
			name: "sm3",
			help: "Chinese National Standard SM3",
			f: AlgorithmF::Normal(sm3),
		},
		Algorithm {
			name: "twox",
			help: "TwoX",
			f: AlgorithmF::WithSeed(twox),
		},
	];
	static ref ALGORITHMS: HashMap<&'static str, &'static Algorithm> =
		RAW_ALGORITHMS.iter().map(|x| (x.name, x)).collect();
	static ref ALGORITHM_HELP: String = "Hash algorithm\n".to_string()
		+ &RAW_ALGORITHMS
			.iter()
			.map(|a| { format!("{}: {}", a.name, a.help) })
			.collect::<Vec<String>>()
			.join("\n");
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![Command {
		app: SubCommand::with_name("hash")
			.about("Hex to hash")
			.arg(
				Arg::with_name("ALGORITHM")
					.long("algo")
					.short("a")
					.help(&ALGORITHM_HELP)
					.takes_value(true)
					.required(true),
			)
			.arg(
				Arg::with_name("KEY")
					.long("key")
					.short("k")
					.help("Key for Blake2b")
					.takes_value(true)
					.required(false),
			)
			.arg(
				Arg::with_name("SEED")
					.long("seed")
					.short("s")
					.help("Seed for twox")
					.takes_value(true)
					.required(false),
			)
			.arg(Arg::with_name("INPUT").required(false).index(1)),
		f: hash,
	}]
}

fn hash(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Convert failed")?.into();

	let a_name = matches.value_of("ALGORITHM").ok_or("Invalid algorithm")?;

	let result = match ALGORITHMS.get(a_name) {
		Some(a) => match a.f {
			AlgorithmF::Normal(f) => (f)(input)?,
			AlgorithmF::WithKey(f) => {
				let key = match matches.value_of("KEY") {
					Some(key) => key.parse::<Hex>().map_err(|_| "Invalid key")?.into(),
					None => vec![],
				};
				(f)(input, key)?
			}
			AlgorithmF::WithSeed(f) => {
				let seed = match matches.value_of("SEED") {
					Some(seed) => seed.parse::<u64>().map_err(|_| "Invalid seed")?,
					None => 0,
				};
				(f)(input, seed)?
			}
		},
		None => return Err("Invalid algorithm".to_string()),
	};

	let result = Hex::from(result).into();

	Ok(vec![result])
}

fn md5(data: Vec<u8>) -> Result<Vec<u8>, String> {
	Ok(md5::compute(data).0.to_vec())
}

fn sha1(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
	context.update(&data);
	let result = context.finish().as_ref().to_vec();
	Ok(result)
}

fn sha2_224(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = Sha224::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_256(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = Sha256::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_384(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = Sha384::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_512(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = Sha512::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_512_224(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = Sha512Trunc224::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha2_512_256(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = Sha512Trunc256::new();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_224(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Sha3_224::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_256(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Sha3_256::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_384(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Sha3_384::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_512(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Sha3_512::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_224(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Keccak224::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_256(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Keccak256::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_384(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Keccak384::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn sha3_k_512(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = sha3::Keccak512::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn ripemd_160(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut hasher = ripemd160::Ripemd160::default();
	hasher.input(data);
	let result = hasher.result().to_vec();
	Ok(result)
}

fn crc_32(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let result = crc32::checksum_ieee(&data);
	let result = result.to_be_bytes().to_vec();

	Ok(result)
}

fn blake2b_160(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, String> {
	blake2b(data, 20, key)
}

fn blake2b_256(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, String> {
	blake2b(data, 32, key)
}

fn blake2b_384(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, String> {
	blake2b(data, 48, key)
}

fn blake2b_512(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, String> {
	blake2b(data, 64, key)
}

fn blake2b(data: Vec<u8>, size: usize, key: Vec<u8>) -> Result<Vec<u8>, String> {
	let mut params = blake2b_simd::Params::new();
	params.hash_length(size);
	if !key.is_empty() {
		params.key(&key);
	}
	let hash = params.to_state().update(&data).finalize();
	Ok(hash.as_bytes().to_vec())
}

fn sm3(data: Vec<u8>) -> Result<Vec<u8>, String> {
	let result = sm3_enc(&data);

	let result: Vec<u8> = result
		.iter()
		.map(|x| x.to_be_bytes().to_vec())
		.flatten()
		.collect();

	Ok(result)
}

fn twox(data: Vec<u8>, seed: u64) -> Result<Vec<u8>, String> {
	use ::core::hash::Hasher;
	let mut h = twox_hash::XxHash::with_seed(seed);
	h.write(&data);
	let r = h.finish();
	use byteorder::{ByteOrder, LittleEndian};
	let mut dest = vec![0u8; 8];
	LittleEndian::write_u64(&mut dest[0..8], r);
	Ok(dest)
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			("hash",
			 vec![
				 Case {
					 desc: "MD5".to_string(),
					 input: vec!["-a", "md5", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x900150983cd24fb0d6963f7d28e17f72"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-1".to_string(),
					 input: vec!["-a", "sha1", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xa9993e364706816aba3e25717850c26c9cd0d89d"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-2 224".to_string(),
					 input: vec!["-a", "sha2_224", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-2 256".to_string(),
					 input: vec!["-a", "sha2_256", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-2 384".to_string(),
					 input: vec!["-a", "sha2_384", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xcb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-2 512".to_string(),
					 input: vec!["-a", "sha2_512", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-2 512 truncate 224".to_string(),
					 input: vec!["-a", "sha2_512_224", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x4634270f707b6a54daae7530460842e20e37ed265ceee9a43e8924aa"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-2 512 truncate 256".to_string(),
					 input: vec!["-a", "sha2_512_256", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 224".to_string(),
					 input: vec!["-a", "sha3_224", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xe642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 256".to_string(),
					 input: vec!["-a", "sha3_256", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 384".to_string(),
					 input: vec!["-a", "sha3_384", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 512".to_string(),
					 input: vec!["-a", "sha3_512", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xb751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 keccak 224".to_string(),
					 input: vec!["-a", "sha3_k_224", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xc30411768506ebe1c2871b1ee2e87d38df342317300a9b97a95ec6a8"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 keccak 256".to_string(),
					 input: vec!["-a", "sha3_k_256", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 keccak 384".to_string(),
					 input: vec!["-a", "sha3_k_384", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xf7df1165f033337be098e7d288ad6a2f74409d7a60b49c36642218de161b1f99f8c681e4afaf31a34db29fb763e3c28e"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "SHA-3 keccak 512".to_string(),
					 input: vec!["-a", "sha3_k_512", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x18587dc2ea106b9a1563e32b3312421ca164c7f1f07bc922a9c83d77cea3a1e5d0c69910739025372dc14ac9642629379540c17e2a65b19d77aa511a9d00bb96"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "RIPEMD-160".to_string(),
					 input: vec!["-a", "ripemd_160", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x8eb208f7e05d987a9b044a8e98c6b087f15a0bfc"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.2.0".to_string(),
				 },
				 Case {
					 desc: "CRC32".to_string(),
					 input: vec!["-a", "crc_32", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x352441c2"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 160".to_string(),
					 input: vec!["-a", "blake2b_160", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x384264f676f39536840523f284921cdc68b6846b"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 160".to_string(),
					 input: vec!["-a", "blake2b_160", "-k", "0x646566", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x6f558b35e06631b03446e7ab87802058d7cd265c"].into_iter().map(Into::into).collect(),
					 is_example: false,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 256".to_string(),
					 input: vec!["-a", "blake2b_256", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xbddd813c634239723171ef3fee98579b94964e3bb1cb3e427262c8c068d52319"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 256".to_string(),
					 input: vec!["-a", "blake2b_256", "-k", "0x646566", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xc3e58ca5dfa559fe4809eeb2f19ee8694a311c5b98f0ebfc241ea2bbbe577d75"].into_iter().map(Into::into).collect(),
					 is_example: false,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 384".to_string(),
					 input: vec!["-a", "blake2b_384", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x6f56a82c8e7ef526dfe182eb5212f7db9df1317e57815dbda46083fc30f54ee6c66ba83be64b302d7cba6ce15bb556f4"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 384".to_string(),
					 input: vec!["-a", "blake2b_384", "-k", "0x646566", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x3ddbbbe27b5a850bff56e0f5041f0e792730c17d648199e353f46bec666c314fa4ecbe31df150595169fc8c521643902"].into_iter().map(Into::into).collect(),
					 is_example: false,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 512".to_string(),
					 input: vec!["-a", "blake2b_512", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0xba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Blake2b 512".to_string(),
					 input: vec!["-a", "blake2b_512", "-k", "0x646566", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x956f2f56e2308b97120bb9f50eefaa5c6a5ae4238a372e308aeb824d3166d869c9a9ba32226d33ba081b235fc45c03852b262d97ce13018c55ed304d302c86b5"].into_iter().map(Into::into).collect(),
					 is_example: false,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "SM3".to_string(),
					 input: vec!["-a", "sm3", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.7.0".to_string(),
				 },
				 Case {
					 desc: "TwoX".to_string(),
					 input: vec!["-a", "twox", "-s", "1", "0x616263"].into_iter().map(Into::into).collect(),
					 output: vec!["0x0889329981caa9be"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.10.0".to_string(),
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
