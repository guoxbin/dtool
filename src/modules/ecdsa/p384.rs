use crate::modules::ecdsa::SignatureFormEnum;
use ring::signature::{EcdsaKeyPair, ECDSA_P384_SHA384_FIXED_SIGNING, ECDSA_P384_SHA384_ASN1_SIGNING, KeyPair, ECDSA_P384_SHA384_FIXED, ECDSA_P384_SHA384_ASN1, VerificationAlgorithm};
use ring::rand::SystemRandom;
use untrusted::Input;

pub fn ec_gk_p384(compress: bool) -> Result<(Vec<u8>, Vec<u8>), String> {
	if compress == true {
		return Err("Compress is not supported".to_string());
	}

	let secret_key = EcdsaKeyPair::generate_private_key(&ECDSA_P384_SHA384_FIXED_SIGNING, &SystemRandom::new()).map_err(|_| "")?;
	let pair = EcdsaKeyPair::from_private_key(&ECDSA_P384_SHA384_FIXED_SIGNING, secret_key.as_ref()).map_err(|_| "Invalid secret key")?;

	let public_key = pair.public_key();
	let public_key = public_key.as_ref().to_vec();

	Ok((secret_key, public_key))
}

pub fn ec_sign_p384(secret_key: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<Vec<u8>, String> {

	let algo = match sig_form{
		SignatureFormEnum::Fixed => &ECDSA_P384_SHA384_FIXED_SIGNING,
		SignatureFormEnum::Der => &ECDSA_P384_SHA384_ASN1_SIGNING,
	};

	let pair = EcdsaKeyPair::from_private_key(&algo, secret_key.as_ref()).map_err(|_| "Invalid secret key")?;
	let sig = pair.sign(&SystemRandom::new(), &message).map_err(|_|"Failed to sign")?;

	Ok(sig.as_ref().to_vec())
}

pub fn ec_verify_p384(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<(), String> {

	let algo = match sig_form{
		SignatureFormEnum::Fixed => &ECDSA_P384_SHA384_FIXED,
		SignatureFormEnum::Der => &ECDSA_P384_SHA384_ASN1,
	};

	let result = algo.verify(Input::from(&public_key),
	                         Input::from(&message), Input::from(&sig))
		.map_err(|e| format!("Invalid signature: {}", e))?;

	Ok(result)
}

pub fn ec_pk_p384(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String> {

	if compress == true {
		return Err("Compress is not supported".to_string());
	}

	let pair = EcdsaKeyPair::from_private_key(&ECDSA_P384_SHA384_FIXED_SIGNING, secret_key.as_ref()).map_err(|_| "Invalid secret key")?;

	let public_key = pair.public_key();
	let public_key = public_key.as_ref().to_vec();

	Ok(public_key)
}
