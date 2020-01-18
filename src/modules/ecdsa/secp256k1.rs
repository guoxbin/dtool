use signatory_secp256k1::{SecretKey, EcdsaSigner, PublicKey, EcdsaVerifier};
use signatory::ecdsa::curve::secp256k1::{Asn1Signature, FixedSignature};
use crate::modules::ecdsa::SignatureFormEnum;
use signatory::public_key::PublicKeyed;
use signatory::signature::{Signer, Verifier, Signature};
use secp256k1::rand::thread_rng;
use secp256k1::Secp256k1;
use crate::modules::base::Hex;

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
