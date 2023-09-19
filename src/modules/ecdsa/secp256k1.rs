use linked_hash_map::LinkedHashMap;
use secp256k1::hashes::sha256;
use secp256k1::rand::thread_rng;
use secp256k1::{ecdsa, Message, PublicKey, Secp256k1, SecretKey};

use crate::modules::ecdsa::SignatureFormEnum;
use crate::modules::Case;

pub fn ec_gk_secp256k1(compress: bool) -> Result<(Vec<u8>, Vec<u8>), String> {
	let (secret_key, public_key) = secp256k1::Secp256k1::new().generate_keypair(&mut thread_rng());

	let secret_key: Vec<u8> = secret_key.as_ref().to_vec();

	let public_key = match compress {
		true => public_key.serialize().to_vec(),
		false => public_key.serialize_uncompressed().to_vec(),
	};
	Ok((secret_key, public_key))
}

pub fn ec_sign_secp256k1(
	secret_key: Vec<u8>,
	message: Vec<u8>,
	sig_form: SignatureFormEnum,
) -> Result<Vec<u8>, String> {
	let secp = Secp256k1::new();
	let message = Message::from_hashed_data::<sha256::Hash>(&message);
	let secret_key =
		SecretKey::from_slice(&secret_key).map_err(|e| format!("Invalid secret key: {}", e))?;
	let signature = secp.sign_ecdsa(&message, &secret_key);

	let signature = match sig_form {
		SignatureFormEnum::Fixed => signature.serialize_compact().to_vec(),
		SignatureFormEnum::Der => signature.serialize_der().as_ref().to_vec(),
	};

	Ok(signature)
}

pub fn ec_verify_secp256k1(
	public_key: Vec<u8>,
	sig: Vec<u8>,
	message: Vec<u8>,
	sig_form: SignatureFormEnum,
) -> Result<(), String> {
	let secp = Secp256k1::new();
	let message = Message::from_hashed_data::<sha256::Hash>(&message);
	let sig = match sig_form {
		SignatureFormEnum::Fixed => ecdsa::Signature::from_compact(&sig),
		SignatureFormEnum::Der => ecdsa::Signature::from_der(&sig),
	}
	.map_err(|e| format!("Invalid signature: {}", e))?;

	let public_key =
		PublicKey::from_slice(&public_key).map_err(|e| format!("Invalid secret key: {}", e))?;
	let result = secp
		.verify_ecdsa(&message, &sig, &public_key)
		.map_err(|e| format!("{}", e));
	result
}

pub fn ec_pk_secp256k1(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String> {
	let secret_key =
		SecretKey::from_slice(&secret_key).map_err(|e| format!("Invalid secret key: {}", e))?;

	let secp = secp256k1::Secp256k1::new();
	let public_key = PublicKey::from_secret_key(&secp, &secret_key);

	let public_key = match compress {
		true => public_key.serialize().to_vec(),
		false => public_key.serialize_uncompressed().to_vec(),
	};

	Ok(public_key)
}

pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
	vec![
        ("ec_gk",
         vec![
             Case {
                 desc: "Secp256k1".to_string(),
                 input: vec!["-c", "secp256k1", "-C"].into_iter().map(Into::into).collect(),
                 output: vec!["(0x9cbe9cd5d7759ca46296f64e3e8211ef5ccaf86b5cb7169711554d1ed2ed68ca, 0x0379ce37925295f3103855da38ee2bf0e06a60ec9d86806d0efd2de3649a74b40d)"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: false,
                 since: "0.7.0".to_string(),
             },
         ]),
        ("ec_sign",
         vec![
             Case {
                 desc: "Secp256k1".to_string(),
                 input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["0x7c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e95364b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.7.0".to_string(),
             },
             Case {
                 desc: "Secp256k1 DER signature form".to_string(),
                 input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "-f", "der", "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["0x304402207c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e953602204b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0"].into_iter().map(Into::into).collect(),
                 is_example: false,
                 is_test: true,
                 since: "0.7.0".to_string(),
             },
         ]),
        ("ec_verify",
         vec![
             Case {
                 desc: "Secp256k1".to_string(),
                 input: vec!["-c", "secp256k1", "-p", "0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60", "-S",
                             "0x7c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e95364b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0",
                             "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["true"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.7.0".to_string(),
             },
             Case {
                 desc: "Secp256k1 DER signature form".to_string(),
                 input: vec!["-c", "secp256k1", "-p", "0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60", "-f", "der", "-S",
                             "0x304402207c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e953602204b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0",
                             "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["true"].into_iter().map(Into::into).collect(),
                 is_example: false,
                 is_test: true,
                 since: "0.7.0".to_string(),
             },
         ]),
        ("ec_pk",
         vec![
             Case {
                 desc: "Secp256k1".to_string(),
                 input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a"].into_iter().map(Into::into).collect(),
                 output: vec!["0x04391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d6035af9529d80a85ebecb1120d1cfaf1591b7c686907b0a3d18858a95e86976747"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.7.0".to_string(),
             },
             Case {
                 desc: "Secp256k1 Compressed public key".to_string(),
                 input: vec!["-c", "secp256k1", "-s", "0x9cb4f775e9b67118242cea15285555c287a7e3d2f86ba238c1fe87284b898e9a", "-C"].into_iter().map(Into::into).collect(),
                 output: vec!["0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3b9618d60"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.7.0".to_string(),
             },
         ]),
    ].into_iter().collect()
}
