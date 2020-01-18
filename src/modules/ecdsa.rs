use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::modules::base::Hex;

mod secp256k1;
mod p256;
mod p384;

struct Curve {
	name: &'static str,
	help: &'static str,
	gk_f: fn(compress: bool) -> Result<(Vec<u8>, Vec<u8>), String>,
	sign_f: fn(secret_key: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<Vec<u8>, String>,
	verify_f: fn(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<(), String>,
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
	static ref RAW_CURVES : Vec<Curve> = vec![
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
		format!("{}: {}", a.name, a.help)
	}).collect::<Vec<String>>().join("\n") + "\n";

	static ref SIGNATURE_FORMS : HashMap<&'static str, &'static SignatureForm> = RAW_SIGNATURE_FORMS.iter().map(|x|(x.name, x)).collect();
	static ref SIGNATURE_FORM_NAMES : Vec<&'static str> = RAW_SIGNATURE_FORMS.iter().map(|x|x.name).collect();
	static ref SIGNATURE_FORM_HELP : String = "Signature form\n".to_string() + &RAW_SIGNATURE_FORMS.iter().map(|a|{
		format!("{}: {}", a.name, a.help)
	}).collect::<Vec<String>>().join("\n") + "\n";
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ec_gk").about("Elliptic-curve generate key pair (Secret key, Public key)")
				.arg(
					Arg::with_name("CURVE")
						.long("curve")
						.short("c").help(&CURVE_HELP)
						.takes_value(true)
						.possible_values(&CURVE_NAMES)
						.required(true))
				.arg(
					Arg::with_name("COMPRESS")
						.long("compress")
						.short("C").help("Compress")
						.required(false)),
			f: ec_gk,
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec!["-c", "secp256k1", "-C"].into_iter().map(Into::into).collect(),
					output: vec!["(0x9cbe9cd5d7759ca46296f64e3e8211ef5ccaf86b5cb7169711554d1ed2ed68ca, 0x0379ce37925295f3103855da38ee2bf0e06a60ec9d86806d0efd2de3649a74b40d)"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: false,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p256", "-C"].into_iter().map(Into::into).collect(),
					output: vec!["(0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9, 0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630)"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: false,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p384", "-C"].into_iter().map(Into::into).collect(),
					output: vec!["(0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f, 0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3)"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: false,
					since: "0.7.0".to_string(),
				},
			],
		},
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
					input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0x7c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e95364b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "-f", "der", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0x304402207c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e953602204b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p256", "-s", "0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0x495f62f272440bd0621d27e97d60c57a0cdaef1cc2434c454eae833bb2111cabb91a79328ee766f720a888b14e0f6037eb8a397dcd9bc9f4c18b9b923a81cc69"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: false,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec!["-c", "p256", "-s", "0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9", "-f", "der", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0x3045022100ed94d4f7022cc2335ef5e34432fed541932e2c2b0c1430e2d51c06e66320302b022002cc2e13e6f5bde7f079a026399e2a6012c5ce4ad2babbe8e1e3444010b72d78"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: false,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p384", "-s", "0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0xa0d387bc5d5de4979750f531f337fd1d04384ab4a9d251a18852c1ce1a16e2e46a2778764d0b3ee090babbc5092ea57a108ddabf9a9fcf8efaad7c0862da2beddde806745c0c3972d738c416d55cfde19b85e39ab54151c87b537c4df7d177ff"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: false,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec!["-c", "p384", "-s", "0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f", "-f", "der", "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["0x3065023100e48b9cd154ecd8dfd138f2e3c5d79af62b3cdc413e52565822edcc96786b03d8e996f132cf793b17c267dc177a5e6525023043dd0485f762b48e3a4a9daeeef57ceff2cf84da6a00b6a65293ee7233efe392ba4514a475476815dddfbbb7ea9e269c"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: false,
					since: "0.7.0".to_string(),
				},
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
					input: vec!["-c", "secp256k1", "-p", "0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60", "-S",
					            "0x7c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e95364b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0",
					            "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec!["-c", "secp256k1", "-p", "0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60", "-f", "der", "-S",
					            "0x304402207c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e953602204b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0",
					            "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p256", "-p", "0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630", "-S",
					            "0x495f62f272440bd0621d27e97d60c57a0cdaef1cc2434c454eae833bb2111cabb91a79328ee766f720a888b14e0f6037eb8a397dcd9bc9f4c18b9b923a81cc69",
					            "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec!["-c", "p256", "-p", "0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630", "-f", "der", "-S",
					            "0x3045022100ed94d4f7022cc2335ef5e34432fed541932e2c2b0c1430e2d51c06e66320302b022002cc2e13e6f5bde7f079a026399e2a6012c5ce4ad2babbe8e1e3444010b72d78",
					            "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p384", "-p", "0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3", "-S",
					            "0xa0d387bc5d5de4979750f531f337fd1d04384ab4a9d251a18852c1ce1a16e2e46a2778764d0b3ee090babbc5092ea57a108ddabf9a9fcf8efaad7c0862da2beddde806745c0c3972d738c416d55cfde19b85e39ab54151c87b537c4df7d177ff",
					            "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "DER signature form".to_string(),
					input: vec!["-c", "p384", "-p", "0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3", "-f", "der", "-S",
					            "0x3065023100e48b9cd154ecd8dfd138f2e3c5d79af62b3cdc413e52565822edcc96786b03d8e996f132cf793b17c267dc177a5e6525023043dd0485f762b48e3a4a9daeeef57ceff2cf84da6a00b6a65293ee7233efe392ba4514a475476815dddfbbb7ea9e269c",
					            "0x616263"].into_iter().map(Into::into).collect(),
					output: vec!["true"].into_iter().map(Into::into).collect(),
					is_example: false,
					is_test: true,
					since: "0.7.0".to_string(),
				},
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
						.required(false)),
			f: ec_pk,
			cases: vec![
				Case {
					desc: "".to_string(),
					input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a"].into_iter().map(Into::into).collect(),
					output: vec!["0x04391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d6035af9529d80a85ebecb1120d1cfaf1591b7c686907b0a3d18858a95e86976747"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "Compressed public key".to_string(),
					input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "-C"].into_iter().map(Into::into).collect(),
					output: vec!["0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p256", "-s", "0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9"].into_iter().map(Into::into).collect(),
					output: vec!["0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
				Case {
					desc: "".to_string(),
					input: vec!["-c", "p384", "-s", "0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f"].into_iter().map(Into::into).collect(),
					output: vec!["0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
					since: "0.7.0".to_string(),
				},
			],
		},
	]
}

fn ec_gk(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let curve = matches.value_of("CURVE").ok_or("Invalid curve")?;

	let curve = CURVES.get(curve).ok_or("Invalid curve")?;

	let compress = matches.is_present("COMPRESS");

	let (private_key, public_key) = (curve.gk_f)(compress)?;

	let (private_key, public_key): (String, String) = (Hex::from(private_key).into(), Hex::from(public_key).into(), );

	let result = format!("({}, {})", private_key, public_key);

	Ok(vec![result])
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

	let compress = matches.is_present("COMPRESS");

	let public_key = (curve.pk_f)(secret_key, compress)?;

	let result = Hex::from(public_key).into();

	Ok(vec![result])
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
