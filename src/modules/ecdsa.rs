use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use signatory_secp256k1::{SecretKey, EcdsaSigner, PublicKey, EcdsaVerifier};
use signatory::ecdsa::curve::secp256k1::{Asn1Signature, FixedSignature};
use signatory::signature::{Signer, Verifier, Signature};
use signatory::public_key::PublicKeyed;
use lazy_static::lazy_static;
use std::collections::HashMap;

struct Curve {
	name: &'static str,
	help: &'static str,
	sign_f: fn(secret_key: Vec<u8>, message: Vec<u8>) -> Result<Vec<u8>, String>,
	verify_f: fn(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>) -> Result<(), String>,
	pk_f: fn(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String>,
}

lazy_static! {
	static ref RAW_CURVES : Vec<Curve> = vec![
		Curve {
			name: "secp256k1",
			help: "Secp256k1",
			sign_f: ec_sign_secp256k1,
			verify_f: ec_verify_secp256k1,
			pk_f: ec_pk_secp256k1,
		},
		Curve {
			name: "p256",
			help: "NIST P-256",
			sign_f: ec_sign_secp256k1,
			verify_f: ec_verify_secp256k1,
			pk_f: ec_pk_secp256k1,
		},
	];

	static ref CURVES : HashMap<&'static str, &'static Curve> = RAW_CURVES.iter().map(|x|(x.name, x)).collect();
	static ref CURVE_NAMES : Vec<&'static str> = RAW_CURVES.iter().map(|x|x.name).collect();
	static ref CURVE_HELP : String = "Curve\n".to_string() + &RAW_CURVES.iter().map(|a|{
		format!("{}: {}\n", a.name, a.help)
	}).collect::<Vec<String>>().join("\n");
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ec_sign").about("Elliptic-curve sign")
				.arg(
					Arg::with_name("INPUT")
						.help("Message (Hex)")
						.required(false)
						.index(1))
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c").help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true))
				.arg(
					Arg::with_name("SECRET_KEY")
						.long("secret-key")
						.short("s").help("Secret key (Private key, Hex)")
						.takes_value(true)
						.required(true)),
			f: ec_sign,
			cases: vec![],
		},
		Command {
			app: SubCommand::with_name("ec_verify").about("Elliptic-curve verify")
				.arg(
					Arg::with_name("INPUT")
						.help("Message (Hex)")
						.required(false)
						.index(1))
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c").help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true))
				.arg(
					Arg::with_name("PUBLIC_KEY")
						.long("public-key")
						.short("p").help("Public key (Hex)")
						.takes_value(true)
						.required(true))
				.arg(
					Arg::with_name("SIGNATURE")
						.long("sig")
						.short("S").help("Signature (Hex)")
						.takes_value(true)
						.required(true)),
			f: ec_verify,
			cases: vec![],
		},
		Command {
			app: SubCommand::with_name("ec_pk").about("Elliptic-curve calculate public key")
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c").help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true))
				.arg(
					Arg::with_name("SECRET_KEY")
						.long("secret-key")
						.short("s").help("Secret key (Private key, Hex)")
						.takes_value(true)
						.required(false))
				.arg(
					Arg::with_name("COMPRESS")
						.long("compress")
						.short("C").help("Compress")
						.takes_value(true)
						.possible_values(&["true", "false"])
						.default_value("true")
						.required(false)),
			f: ec_pk,
			cases: vec![],
		},
	]
}

fn ec_sign(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let secret_key = matches.value_of("SECRET_KEY").ok_or("Invalid secret key")?;
	let secret_key = hex::decode(secret_key.trim_start_matches("0x")).map_err(|_| "Invalid secret key")?;

	let input = base::input_string(matches)?;
	let input = hex::decode(input.trim_start_matches("0x")).map_err(|_| "Invalid input")?;

	let sig = (curve.sign_f)(secret_key, input)?;

	let result = format!("0x{}", hex::encode(sig));

	Ok(vec![result])
}

fn ec_verify(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let public_key = matches.value_of("PUBLIC_KEY").ok_or("Invalid public key")?;
	let public_key = hex::decode(public_key.trim_start_matches("0x")).map_err(|_| "Invalid secret key")?;

	let sig = matches.value_of("SIGNATURE").ok_or("Invalid signature")?;
	let sig = hex::decode(sig.trim_start_matches("0x")).map_err(|_| "Invalid signature")?;

	let input = base::input_string(matches)?;
	let input = hex::decode(input.trim_start_matches("0x")).map_err(|_| "Invalid input")?;

	(curve.verify_f)(public_key, sig, input)?;

	let result = "true".to_string();

	Ok(vec![result])
}

fn ec_pk(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let secret_key = matches.value_of("SECRET_KEY").ok_or("Invalid secret key")?;
	let secret_key = hex::decode(secret_key.trim_start_matches("0x")).map_err(|_| "Invalid secret key")?;

	let compress: bool = matches.value_of("COMPRESS").ok_or("Invalid compress")?.parse().map_err(|_| "Invalid compress")?;

	let public_key = (curve.pk_f)(secret_key, compress)?;

	let result = format!("0x{}", hex::encode(public_key));

	Ok(vec![result])
}

fn ec_sign_secp256k1(secret_key: Vec<u8>, message: Vec<u8>) -> Result<Vec<u8>, String> {
	let secret_key = SecretKey::from_bytes(secret_key).map_err(|e| format!("Invalid secret key: {}", e))?;
	let signer = EcdsaSigner::from(&secret_key);

	let signature: FixedSignature = signer.sign(&message);

	let signature = signature.as_ref().to_vec();

	Ok(signature)
}

fn ec_verify_secp256k1(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>) -> Result<(), String> {
	let public_key = PublicKey::from_bytes(public_key).ok_or("Invalid public key")?;
	let verifier = EcdsaVerifier::from(&public_key);

	let sig = FixedSignature::from_bytes(sig).map_err(|e| format!("Invalid signature: {}", e))?;
	verifier.verify(&message, &sig).map_err(|e| format!("{}", e))
}

fn ec_pk_secp256k1(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String> {
	let secret_key = SecretKey::from_bytes(secret_key).map_err(|e| format!("Invalid secret key: {}", e))?;
	let signer = EcdsaSigner::from(&secret_key);

	let public_key: PublicKey = signer.public_key().map_err(|_| "Failed")?;

	let public_key = public_key.as_bytes();

	let public_key = match compress {
		true => public_key.to_vec(),
		false => {
			let public_key = secp256k1::PublicKey::from_slice(public_key).expect("Should be valid public key; qed");
			public_key.serialize_uncompressed().to_vec()
		}
	};

	Ok(public_key)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::base::test::test_commands;

	#[test]
	fn test_cases() {
		test_commands(&commands());
	}
}
