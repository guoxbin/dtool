use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use signatory_secp256k1::{SecretKey, EcdsaSigner, PublicKey, EcdsaVerifier};
use signatory::ecdsa::curve::secp256k1::{Asn1Signature, FixedSignature};
use signatory::signature::{Signer, Verifier, Signature};
use signatory::public_key::PublicKeyed;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::modules::base::Hex;

struct Curve {
	name: &'static str,
	help: &'static str,
	sign_f: fn(secret_key: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<Vec<u8>, String>,
	verify_f: fn(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<(), String>,
	pk_f: fn(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String>,
}

#[derive(Clone)]
enum SignatureFormEnum{
	Der,
	Fixed,
}

struct SignatureForm {
	name: &'static str,
	help: &'static str,
	e: SignatureFormEnum,
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

	static ref RAW_SIGNATURE_FORMS : Vec<SignatureForm> = vec![
		SignatureForm{
			name: "der",
			help: "ASN1 DER",
			e: SignatureFormEnum::Der,
		},
		SignatureForm{
			name: "fixed",
			help: "Fixed",
			e: SignatureFormEnum::Fixed,
		},
	];

	static ref CURVES : HashMap<&'static str, &'static Curve> = RAW_CURVES.iter().map(|x|(x.name, x)).collect();
	static ref CURVE_NAMES : Vec<&'static str> = RAW_CURVES.iter().map(|x|x.name).collect();
	static ref CURVE_HELP : String = "Curve\n".to_string() + &RAW_CURVES.iter().map(|a|{
		format!("{}: {}\n", a.name, a.help)
	}).collect::<Vec<String>>().join("\n");

	static ref SIGNATURE_FORMS : HashMap<&'static str, &'static SignatureForm> = RAW_SIGNATURE_FORMS.iter().map(|x|(x.name, x)).collect();
	static ref SIGNATURE_FORM_NAMES : Vec<&'static str> = RAW_SIGNATURE_FORMS.iter().map(|x|x.name).collect();
	static ref SIGNATURE_FORM_HELP : String = "Signature form\n".to_string() + &RAW_SIGNATURE_FORMS.iter().map(|a|{
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
						.required(true))
				.arg(
					Arg::with_name("SIGNATURE_FORM")
						.long("sig-form")
						.short("f").help(&SIGNATURE_FORM_HELP)
						.takes_value(true)
						.possible_values(&SIGNATURE_FORM_NAMES)
						.default_value("fixed")
						.required(false)),
			f: ec_sign,
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec![ "-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0x7c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e95364b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec![ "-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "-f", "der", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0x304402207c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e953602204b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.7.0".to_string(),
				}
			],
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
						.required(true))
				.arg(
					Arg::with_name("SIGNATURE_FORM")
						.long("sig-form")
						.short("f").help(&SIGNATURE_FORM_HELP)
						.takes_value(true)
						.possible_values(&SIGNATURE_FORM_NAMES)
						.default_value("fixed")
						.required(false)),
			f: ec_verify,
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec![ "-c", "secp256k1", "-p", "0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60", "-S",
					             "0x7c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e95364b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0" ,
					             "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec![ "-c", "secp256k1", "-p", "0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60", "-f", "der", "-S",
					             "0x304402207c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e953602204b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0",
					             "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.7.0".to_string(),
				}
			],
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
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec![ "-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a"].into_iter().map(Into::into).collect(),
					output: vec!["0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "Uncompressed public key".to_string(),
					input: vec![ "-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a"].into_iter().map(Into::into).collect(),
					output: vec!["0x04391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d6035af9529d80a85ebecb1120d1cfaf1591b7c686907b0a3d18858a95e86976747"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
			],
		},
	]
}

fn ec_sign(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let secret_key = matches.value_of("SECRET_KEY").ok_or("Invalid secret key")?;
	let secret_key: Vec<u8> = secret_key.parse::<Hex>().map_err(|_| "Invalid secret key")?.into();

	let input = base::input_string(matches)?;
	let input: Vec<u8> = input.parse::<Hex>().map_err(|_| "Invalid input")?.into();

	let sig_form = matches.value_of("SIGNATURE_FORM").ok_or("Invalid signature form")?;
	let sig_form = SIGNATURE_FORMS.get(sig_form).ok_or("Invalid signature form")?.e.clone();

	let sig = (curve.sign_f)(secret_key, input, sig_form)?;

	let result = Hex::from(sig).into();

	Ok(vec![result])
}

fn ec_verify(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let public_key = matches.value_of("PUBLIC_KEY").ok_or("Invalid public key")?;
	let public_key: Vec<u8> = public_key.parse::<Hex>().map_err(|_| "Invalid secret key")?.into();

	let sig_form = matches.value_of("SIGNATURE_FORM").ok_or("Invalid signature form")?;
	let sig_form = SIGNATURE_FORMS.get(sig_form).ok_or("Invalid signature form")?.e.clone();

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
	let secret_key: Vec<u8> = secret_key.parse::<Hex>().map_err(|_| "Invalid secret key")?.into();

	let compress: bool = matches.value_of("COMPRESS").ok_or("Invalid compress")?.parse().map_err(|_| "Invalid compress")?;

	let public_key = (curve.pk_f)(secret_key, compress)?;

	let result = Hex::from(public_key).into();

	Ok(vec![result])
}

fn ec_sign_secp256k1(secret_key: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<Vec<u8>, String> {
	let secret_key = SecretKey::from_bytes(secret_key).map_err(|e| format!("Invalid secret key: {}", e))?;
	let signer = EcdsaSigner::from(&secret_key);

	let signature = match sig_form{
		SignatureFormEnum::Fixed => {
			let signature: FixedSignature = signer.sign(&message);
			signature.as_ref().to_vec()
		},
		SignatureFormEnum::Der => {
			let signature: Asn1Signature = signer.sign(&message);
			signature.as_ref().to_vec()
		},
	};

	Ok(signature)
}

fn ec_verify_secp256k1(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<(), String> {
	let public_key = PublicKey::from_bytes(public_key).ok_or("Invalid public key")?;
	let verifier = EcdsaVerifier::from(&public_key);

	let result = match sig_form{
		SignatureFormEnum::Fixed => {
			let sig = FixedSignature::from_bytes(sig).map_err(|e| format!("Invalid signature: {}", e))?;
			verifier.verify(&message, &sig).map_err(|e| format!("{}", e))
		},
		SignatureFormEnum::Der => {
			let sig = Asn1Signature::from_bytes(sig).map_err(|e| format!("Invalid signature: {}", e))?;
			verifier.verify(&message, &sig).map_err(|e| format!("{}", e))
		},
	};
	result
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
