use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};

mod ed25519;

pub enum AltSecretKey {
	MiniSecretKey(Vec<u8>),
	SecretKey(Vec<u8>),
}

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "EdDSA (Ed25519)".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ed_gk")
				.about("EdDSA generate key pair (Mini secret key, Public key)"),
			f: ed_gk,
		},
		Command {
			app: SubCommand::with_name("ed_sign")
				.about("EdDSA sign")
				.arg(
					Arg::with_name("INPUT")
						.help("Message (Hex)")
						.required(false)
						.index(1),
				)
				.arg(
					Arg::with_name("MINI_SECRET_KEY")
						.long("mini-secret-key")
						.short("m")
						.help("Mini secret key (Mini private key, Hex)")
						.takes_value(true)
						.required(false),
				)
				.arg(
					Arg::with_name("SECRET_KEY")
						.long("secret-key")
						.short("s")
						.help("Secret key (Private key, Hex)")
						.takes_value(true)
						.required(false),
				),
			f: ed_sign,
		},
		Command {
			app: SubCommand::with_name("ed_verify")
				.about("EdDSA verify")
				.arg(
					Arg::with_name("INPUT")
						.help("Message (Hex)")
						.required(false)
						.index(1),
				)
				.arg(
					Arg::with_name("PUBLIC_KEY")
						.long("public-key")
						.short("p")
						.help("Public key (Hex)")
						.takes_value(true)
						.required(true),
				)
				.arg(
					Arg::with_name("SIGNATURE")
						.long("sig")
						.short("S")
						.help("Signature (Hex)")
						.takes_value(true)
						.required(true),
				),
			f: ed_verify,
		},
		Command {
			app: SubCommand::with_name("ed_sk")
				.about("EdDSA calculate secret key from mini secret key")
				.arg(
					Arg::with_name("MINI_SECRET_KEY")
						.long("mini-secret-key")
						.short("m")
						.help("Mini secret key (Mini private key, Hex)")
						.takes_value(true)
						.required(true),
				),

			f: ed_sk,
		},
		Command {
			app: SubCommand::with_name("ed_pk")
				.about("EdDSA calculate public key")
				.arg(
					Arg::with_name("MINI_SECRET_KEY")
						.long("mini-secret-key")
						.short("m")
						.help("Mini secret key (Mini private key, Hex)")
						.takes_value(true)
						.required(false),
				)
				.arg(
					Arg::with_name("SECRET_KEY")
						.long("secret-key")
						.short("s")
						.help("Secret key (Private key, Hex)")
						.takes_value(true)
						.required(false),
				),
			f: ed_pk,
		},
	]
}

fn ed_gk(_matches: &ArgMatches) -> Result<Vec<String>, String> {
	let (private_key, public_key) = ed25519::ed_gk_ed25519()?;

	let (private_key, public_key): (String, String) =
		(Hex::from(private_key).into(), Hex::from(public_key).into());

	let result = format!("({}, {})", private_key, public_key);

	Ok(vec![result])
}

fn ed_sign(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let secret_key = get_alt_secret_key(matches)?;

	let input = base::input_string(matches)?;
	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	let sig = ed25519::ed_sign_ed25519(secret_key, input)?;

	let result = Hex::from(sig).into();

	Ok(vec![result])
}

fn ed_verify(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let public_key = matches.value_of("PUBLIC_KEY").ok_or("Invalid public key")?;
	let public_key: Vec<u8> = public_key
		.parse::<Hex>()
		.map_err(|_| "Invalid secret key")?
		.into();

	let sig = matches.value_of("SIGNATURE").ok_or("Invalid signature")?;
	let sig: Vec<u8> = sig.parse::<Hex>().map_err(|_| "Invalid signature")?.into();

	let input = base::input_string(matches)?;
	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	ed25519::ed_verify_ed25519(public_key, sig, input)?;

	let result = "true".to_string();

	Ok(vec![result])
}

fn ed_sk(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let mini_secret_key = matches
		.value_of("MINI_SECRET_KEY")
		.ok_or("Invalid mini secret key")?;
	let mini_secret_key: Vec<u8> = mini_secret_key
		.parse::<Hex>()
		.map_err(|_| "Invalid mini secret key")?
		.into();

	let secret_key = ed25519::ed_sk_ed25519(mini_secret_key)?;

	let result = Hex::from(secret_key).into();

	Ok(vec![result])
}

fn ed_pk(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let secret_key = get_alt_secret_key(matches)?;

	let public_key = ed25519::ed_pk_ed25519(secret_key)?;

	let result = Hex::from(public_key).into();

	Ok(vec![result])
}

fn get_alt_secret_key(matches: &ArgMatches) -> Result<AltSecretKey, String> {
	if matches.is_present("MINI_SECRET_KEY") {
		let secret_key = matches
			.value_of("MINI_SECRET_KEY")
			.ok_or("Invalid mini secret key")?;
		let secret_key: Vec<u8> = secret_key
			.parse::<Hex>()
			.map_err(|_| "Invalid mini secret key")?
			.into();
		Ok(AltSecretKey::MiniSecretKey(secret_key))
	} else if matches.is_present("SECRET_KEY") {
		let secret_key = matches.value_of("SECRET_KEY").ok_or("Invalid secret key")?;
		let secret_key: Vec<u8> = secret_key
			.parse::<Hex>()
			.map_err(|_| "Invalid secret key")?
			.into();
		Ok(AltSecretKey::SecretKey(secret_key))
	} else {
		Err("Mini secret key or secret key should be provided".to_string())
	}
}

mod cases {
	use super::ed25519;
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;
	use std::iter::empty;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		empty()
			.chain(ed25519::cases())
			.fold(LinkedHashMap::new(), |mut map, (name, mut cases)| {
				let list = map.entry(name).or_insert(vec![]);
				list.append(&mut cases);
				map
			})
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
