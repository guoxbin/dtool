use crate::modules::ecdsa::SignatureFormEnum;
use crate::modules::Case;
use linked_hash_map::LinkedHashMap;
use ring::rand::SystemRandom;
use ring::signature::{
	EcdsaKeyPair, KeyPair, VerificationAlgorithm, ECDSA_P256_SHA256_ASN1,
	ECDSA_P256_SHA256_ASN1_SIGNING, ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING,
};
use untrusted::Input;

pub fn ec_gk_p256(compress: bool) -> Result<(Vec<u8>, Vec<u8>), String> {
	if compress == true {
		return Err("Compress is not supported".to_string());
	}

	let secret_key =
		EcdsaKeyPair::generate_private_key(&ECDSA_P256_SHA256_FIXED_SIGNING, &SystemRandom::new())
			.map_err(|_| "")?;
	let pair =
		EcdsaKeyPair::from_private_key(&ECDSA_P256_SHA256_FIXED_SIGNING, secret_key.as_ref())
			.map_err(|_| "Invalid secret key")?;

	let public_key = pair.public_key();
	let public_key = public_key.as_ref().to_vec();

	Ok((secret_key, public_key))
}

pub fn ec_sign_p256(
	secret_key: Vec<u8>,
	message: Vec<u8>,
	sig_form: SignatureFormEnum,
) -> Result<Vec<u8>, String> {
	let algo = match sig_form {
		SignatureFormEnum::Fixed => &ECDSA_P256_SHA256_FIXED_SIGNING,
		SignatureFormEnum::Der => &ECDSA_P256_SHA256_ASN1_SIGNING,
	};

	let pair = EcdsaKeyPair::from_private_key(&algo, secret_key.as_ref())
		.map_err(|_| "Invalid secret key")?;
	let sig = pair
		.sign(&SystemRandom::new(), &message)
		.map_err(|_| "Failed to sign")?;

	Ok(sig.as_ref().to_vec())
}

pub fn ec_verify_p256(
	public_key: Vec<u8>,
	sig: Vec<u8>,
	message: Vec<u8>,
	sig_form: SignatureFormEnum,
) -> Result<(), String> {
	let algo = match sig_form {
		SignatureFormEnum::Fixed => &ECDSA_P256_SHA256_FIXED,
		SignatureFormEnum::Der => &ECDSA_P256_SHA256_ASN1,
	};

	let result = algo
		.verify(
			Input::from(&public_key),
			Input::from(&message),
			Input::from(&sig),
		)
		.map_err(|e| format!("Invalid signature: {}", e))?;

	Ok(result)
}

pub fn ec_pk_p256(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String> {
	if compress == true {
		return Err("Compress is not supported".to_string());
	}

	let pair =
		EcdsaKeyPair::from_private_key(&ECDSA_P256_SHA256_FIXED_SIGNING, secret_key.as_ref())
			.map_err(|_| "Invalid secret key")?;

	let public_key = pair.public_key();
	let public_key = public_key.as_ref().to_vec();

	Ok(public_key)
}

pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
	vec![
		("ec_gk",
		 vec![
			 Case {
				 desc: "P-256".to_string(),
				 input: vec!["-c", "p256"].into_iter().map(Into::into).collect(),
				 output: vec!["(0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9, 0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630)"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
		 ]),
		("ec_sign",
		 vec![
			 Case {
				 desc: "P-256".to_string(),
				 input: vec!["-c", "p256", "-s", "0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9", "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["0x495f62f272440bd0621d27e97d60c57a0cdaef1cc2434c454eae833bb2111cabb91a79328ee766f720a888b14e0f6037eb8a397dcd9bc9f4c18b9b923a81cc69"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
			 Case {
				 desc: "P-256 DER signature form".to_string(),
				 input: vec!["-c", "p256", "-s", "0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9", "-f", "der", "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["0x3045022100ed94d4f7022cc2335ef5e34432fed541932e2c2b0c1430e2d51c06e66320302b022002cc2e13e6f5bde7f079a026399e2a6012c5ce4ad2babbe8e1e3444010b72d78"].into_iter().map(Into::into).collect(),
				 is_example: false,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
		 ]),
		("ec_verify",
		 vec![
			 Case {
				 desc: "P-256".to_string(),
				 input: vec!["-c", "p256", "-p", "0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630", "-S",
				             "0x495f62f272440bd0621d27e97d60c57a0cdaef1cc2434c454eae833bb2111cabb91a79328ee766f720a888b14e0f6037eb8a397dcd9bc9f4c18b9b923a81cc69",
				             "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["true"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: true,
				 since: "0.7.0".to_string(),
			 },
			 Case {
				 desc: "P-256 DER signature form".to_string(),
				 input: vec!["-c", "p256", "-p", "0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630", "-f", "der", "-S",
				             "0x3045022100ed94d4f7022cc2335ef5e34432fed541932e2c2b0c1430e2d51c06e66320302b022002cc2e13e6f5bde7f079a026399e2a6012c5ce4ad2babbe8e1e3444010b72d78",
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
				 desc: "P-256".to_string(),
				 input: vec!["-c", "p256", "-s", "0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed5a611c9"].into_iter().map(Into::into).collect(),
				 output: vec!["0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2839bdbbee630"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: true,
				 since: "0.7.0".to_string(),
			 },
		 ]),
	].into_iter().collect()
}
