use crate::modules::base::Hex;
use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use lazy_static::lazy_static;
use std::collections::HashMap;

mod p256;
mod p384;
mod secp256k1;
mod sm2;

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "ECDSA (Secp256k1, NIST P-256, NIST P-384, SM2)".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

struct Curve {
	name: &'static str,
	help: &'static str,
	gk_f: fn(compress: bool) -> Result<(Vec<u8>, Vec<u8>), String>,
	sign_f: fn(
		secret_key: Vec<u8>,
		message: Vec<u8>,
		sig_form: SignatureFormEnum,
	) -> Result<Vec<u8>, String>,
	verify_f: fn(
		public_key: Vec<u8>,
		sig: Vec<u8>,
		message: Vec<u8>,
		sig_form: SignatureFormEnum,
	) -> Result<(), String>,
	pk_f: fn(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String>,
}

#[derive(Clone)]
pub enum SignatureFormEnum {
	Der,
	Fixed,
}

struct SignatureForm {
	name: &'static str,
	help: &'static str,
	e: SignatureFormEnum,
}

lazy_static! {
	static ref RAW_CURVES: Vec<Curve> = vec![
		Curve {
			name: "secp256k1",
			help: "Secp256k1",
			gk_f: secp256k1::ec_gk_secp256k1,
			sign_f: secp256k1::ec_sign_secp256k1,
			verify_f: secp256k1::ec_verify_secp256k1,
			pk_f: secp256k1::ec_pk_secp256k1,
		},
		Curve {
			name: "p256",
			help: "NIST P-256",
			gk_f: p256::ec_gk_p256,
			sign_f: p256::ec_sign_p256,
			verify_f: p256::ec_verify_p256,
			pk_f: p256::ec_pk_p256,
		},
		Curve {
			name: "p384",
			help: "NIST P-384",
			gk_f: p384::ec_gk_p384,
			sign_f: p384::ec_sign_p384,
			verify_f: p384::ec_verify_p384,
			pk_f: p384::ec_pk_p384,
		},
		Curve {
			name: "sm2",
			help: "Chinese National Standard SM2",
			gk_f: sm2::ec_gk_sm2,
			sign_f: sm2::ec_sign_sm2,
			verify_f: sm2::ec_verify_sm2,
			pk_f: sm2::ec_pk_sm2,
		},
	];
	static ref RAW_SIGNATURE_FORMS: Vec<SignatureForm> = vec![
		SignatureForm {
			name: "der",
			help: "ASN1 DER",
			e: SignatureFormEnum::Der,
		},
		SignatureForm {
			name: "fixed",
			help: "Fixed",
			e: SignatureFormEnum::Fixed,
		},
	];
	static ref CURVES: HashMap<&'static str, &'static Curve> =
		RAW_CURVES.iter().map(|x| (x.name, x)).collect();
	static ref CURVE_NAMES: Vec<&'static str> = RAW_CURVES.iter().map(|x| x.name).collect();
	static ref CURVE_HELP: String = "Curve\n".to_string()
		+ &RAW_CURVES
			.iter()
			.map(|a| { format!("{}: {}", a.name, a.help) })
			.collect::<Vec<String>>()
			.join("\n")
		+ "\n";
	static ref SIGNATURE_FORMS: HashMap<&'static str, &'static SignatureForm> =
		RAW_SIGNATURE_FORMS.iter().map(|x| (x.name, x)).collect();
	static ref SIGNATURE_FORM_NAMES: Vec<&'static str> =
		RAW_SIGNATURE_FORMS.iter().map(|x| x.name).collect();
	static ref SIGNATURE_FORM_HELP: String = "Signature form\n".to_string()
		+ &RAW_SIGNATURE_FORMS
			.iter()
			.map(|a| { format!("{}: {}", a.name, a.help) })
			.collect::<Vec<String>>()
			.join("\n")
		+ "\n";
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ec_gk")
				.about("Elliptic-curve generate key pair (Secret key, Public key)")
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c")
						.help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true),
				)
				.arg(
					Arg::with_name("COMPRESS")
						.long("compress")
						.short("C")
						.help("Compress")
						.required(false),
				),
			f: ec_gk,
		},
		Command {
			app: SubCommand::with_name("ec_sign")
				.about("Elliptic-curve sign")
				.arg(
					Arg::with_name("INPUT")
						.help("Message (Hex)")
						.required(false)
						.index(1),
				)
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c")
						.help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true),
				)
				.arg(
					Arg::with_name("SECRET_KEY")
						.long("secret-key")
						.short("s")
						.help("Secret key (Private key, Hex)")
						.takes_value(true)
						.required(true),
				)
				.arg(
					Arg::with_name("SIGNATURE_FORM")
						.long("sig-form")
						.short("f")
						.help(&SIGNATURE_FORM_HELP)
						.takes_value(true)
						.possible_values(&SIGNATURE_FORM_NAMES)
						.default_value("fixed")
						.required(false),
				),
			f: ec_sign,
		},
		Command {
			app: SubCommand::with_name("ec_verify")
				.about("Elliptic-curve verify")
				.arg(
					Arg::with_name("INPUT")
						.help("Message (Hex)")
						.required(false)
						.index(1),
				)
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c")
						.help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true),
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
				)
				.arg(
					Arg::with_name("SIGNATURE_FORM")
						.long("sig-form")
						.short("f")
						.help(&SIGNATURE_FORM_HELP)
						.takes_value(true)
						.possible_values(&SIGNATURE_FORM_NAMES)
						.default_value("fixed")
						.required(false),
				),
			f: ec_verify,
		},
		Command {
			app: SubCommand::with_name("ec_pk")
				.about("Elliptic-curve calculate public key")
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c")
						.help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true),
				)
				.arg(
					Arg::with_name("SECRET_KEY")
						.long("secret-key")
						.short("s")
						.help("Secret key (Private key, Hex)")
						.takes_value(true)
						.required(false),
				)
				.arg(
					Arg::with_name("COMPRESS")
						.long("compress")
						.short("C")
						.help("Compress")
						.required(false),
				),
			f: ec_pk,
		},
	]
}

fn ec_gk(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let compress = matches.is_present("COMPRESS");

	let (private_key, public_key) = (curve.gk_f)(compress)?;

	let (private_key, public_key): (String, String) =
		(Hex::from(private_key).into(), Hex::from(public_key).into());

	let result = format!("({}, {})", private_key, public_key);

	Ok(vec![result])
}

fn ec_sign(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let secret_key = matches.value_of("SECRET_KEY").ok_or("Invalid secret key")?;
	let secret_key: Vec<u8> = secret_key
		.parse::<Hex>()
		.map_err(|_| "Invalid secret key")?
		.into();

	let input = base::input_string(matches)?;
	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	let sig_form = matches
		.value_of("SIGNATURE_FORM")
		.ok_or("Invalid signature form")?;
	let sig_form = SIGNATURE_FORMS
		.get(sig_form)
		.ok_or("Invalid signature form")?
		.e
		.clone();

	let sig = (curve.sign_f)(secret_key, input, sig_form)?;

	let result = Hex::from(sig).into();

	Ok(vec![result])
}

fn ec_verify(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let public_key = matches.value_of("PUBLIC_KEY").ok_or("Invalid public key")?;
	let public_key: Vec<u8> = public_key
		.parse::<Hex>()
		.map_err(|_| "Invalid secret key")?
		.into();

	let sig_form = matches
		.value_of("SIGNATURE_FORM")
		.ok_or("Invalid signature form")?;
	let sig_form = SIGNATURE_FORMS
		.get(sig_form)
		.ok_or("Invalid signature form")?
		.e
		.clone();

	let sig = matches.value_of("SIGNATURE").ok_or("Invalid signature")?;
	let sig: Vec<u8> = sig.parse::<Hex>().map_err(|_| "Invalid signature")?.into();

	let input = base::input_string(matches)?;
	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	(curve.verify_f)(public_key, sig, input, sig_form)?;

	let result = "true".to_string();

	Ok(vec![result])
}

fn ec_pk(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let secret_key = matches.value_of("SECRET_KEY").ok_or("Invalid secret key")?;
	let secret_key: Vec<u8> = secret_key
		.parse::<Hex>()
		.map_err(|_| "Invalid secret key")?
		.into();

	let compress = matches.is_present("COMPRESS");

	let public_key = (curve.pk_f)(secret_key, compress)?;

	let result = Hex::from(public_key).into();

	Ok(vec![result])
}

mod cases {
	use super::p256;
	use super::p384;
	use super::secp256k1;
	use super::sm2;
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;
	use std::iter::empty;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		empty()
			.chain(secp256k1::cases())
			.chain(p256::cases())
			.chain(p384::cases())
			.chain(sm2::cases())
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
