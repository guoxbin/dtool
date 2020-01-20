use signatory_secp256k1::{SecretKey, EcdsaSigner, PublicKey, EcdsaVerifier};
use signatory::ecdsa::curve::secp256k1::{Asn1Signature, FixedSignature};
use crate::modules::ecdsa::SignatureFormEnum;
use signatory::public_key::PublicKeyed;
use signatory::signature::{Signer, Verifier, Signature};
use secp256k1::rand::thread_rng;
use secp256k1::Secp256k1;
use crate::modules::base::Hex;
use crate::modules::Case;
use linked_hash_map::LinkedHashMap;

pub fn ec_gk_secp256k1(compress: bool) -> Result<(Vec<u8>, Vec<u8>), String> {
	let (secret_key, public_key) = Secp256k1::new().generate_keypair(&mut thread_rng());

	let secret_key: Vec<u8> = secret_key.to_string().parse::<Hex>().map_err(|_| "Invalid secret key")?.into();

	let public_key = match compress {
		true => public_key.serialize().to_vec(),
		false => public_key.serialize_uncompressed().to_vec(),
	};
	Ok((secret_key, public_key))
}

pub fn ec_sign_secp256k1(secret_key: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<Vec<u8>, String> {
	let secret_key = SecretKey::from_bytes(secret_key).map_err(|e| format!("Invalid secret key: {}", e))?;
	let signer = EcdsaSigner::from(&secret_key);

	let signature = match sig_form {
		SignatureFormEnum::Fixed => {
			let signature: FixedSignature = signer.sign(&message);
			signature.as_ref().to_vec()
		}
		SignatureFormEnum::Der => {
			let signature: Asn1Signature = signer.sign(&message);
			signature.as_ref().to_vec()
		}
	};

	Ok(signature)
}

pub fn ec_verify_secp256k1(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<(), String> {
	let public_key = PublicKey::from_bytes(public_key).ok_or("Invalid public key")?;
	let verifier = EcdsaVerifier::from(&public_key);

	let result = match sig_form {
		SignatureFormEnum::Fixed => {
			let sig = FixedSignature::from_bytes(sig).map_err(|e| format!("Invalid signature: {}", e))?;
			verifier.verify(&message, &sig).map_err(|e| format!("{}", e))
		}
		SignatureFormEnum::Der => {
			let sig = Asn1Signature::from_bytes(sig).map_err(|e| format!("Invalid signature: {}", e))?;
			verifier.verify(&message, &sig).map_err(|e| format!("{}", e))
		}
	};
	result
}

pub fn ec_pk_secp256k1(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String> {
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
