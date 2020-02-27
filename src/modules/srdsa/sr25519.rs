use crate::modules::srdsa::AltSecretKey;
use crate::modules::Case;
use linked_hash_map::LinkedHashMap;
use schnorrkel::{ExpansionMode, Keypair};
use secp256k1::rand::thread_rng;

pub fn sr_gk_sr25519() -> Result<(Vec<u8>, Vec<u8>), String> {
	let mini_secret_key = schnorrkel::MiniSecretKey::generate_with(&mut thread_rng());

	let secret_key = mini_secret_key.expand(ExpansionMode::Ed25519);

	let public_key = secret_key.to_public().as_ref().to_vec();

	let secret_key = mini_secret_key.as_bytes().to_vec();

	Ok((secret_key, public_key))
}

pub fn sr_sign_sr25519(secret_key: AltSecretKey, message: Vec<u8>) -> Result<Vec<u8>, String> {
	let key_pair = get_key_pair(secret_key)?;

	let signature = key_pair.sign_simple(&[], &message);

	let signature = signature.to_bytes().to_vec();

	Ok(signature)
}

pub fn sr_verify_sr25519(
	public_key: Vec<u8>,
	sig: Vec<u8>,
	message: Vec<u8>,
) -> Result<(), String> {
	let public_key =
		schnorrkel::PublicKey::from_bytes(&public_key).map_err(|_| "Invalid public key")?;

	let signature = schnorrkel::Signature::from_bytes(&sig).map_err(|_| "Invalid signature")?;

	let result = public_key
		.verify_simple(&[], &message, &signature)
		.map_err(|e| format!("Invalid signature: {}", e))?;

	Ok(result)
}

pub fn sr_sk_sr25519(mini_secret_key: Vec<u8>) -> Result<Vec<u8>, String> {
	let mini_secret_key = schnorrkel::MiniSecretKey::from_bytes(&mini_secret_key)
		.map_err(|_| "Invalid mini secret key")?;
	let key_pair = mini_secret_key.expand_to_keypair(ExpansionMode::Ed25519);

	let secret_key = &key_pair.secret;
	let secret_key = secret_key.to_bytes().to_vec();

	Ok(secret_key)
}

pub fn sr_pk_sr25519(secret_key: AltSecretKey) -> Result<Vec<u8>, String> {
	let key_pair = get_key_pair(secret_key)?;

	let public_key = key_pair.public;
	let public_key = public_key.as_ref().to_vec();

	Ok(public_key)
}

fn get_key_pair(secret_key: AltSecretKey) -> Result<Keypair, String> {
	let key_pair = match secret_key {
		AltSecretKey::MiniSecretKey(key) => {
			let mini_secret_key = schnorrkel::MiniSecretKey::from_bytes(&key)
				.map_err(|_| "Invalid mini secret key")?;
			let key_pair = mini_secret_key.expand_to_keypair(ExpansionMode::Ed25519);
			key_pair
		}
		AltSecretKey::SecretKey(key) => {
			let secret_key =
				schnorrkel::SecretKey::from_bytes(&key).map_err(|_| "Invalid secret key")?;
			let key_pair = secret_key.to_keypair();
			key_pair
		}
	};
	Ok(key_pair)
}

pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
	vec![
        ("sr_gk",
         vec![
             Case {
                 desc: "".to_string(),
                 input: Vec::<String>::new().into_iter().map(Into::into).collect(),
                 output: vec!["(0xc243239f434f7a4b0ab8d4600537001e6479c807c3d3623f99c8ad9f2a588837, 0x6a8ee649b31efe7aabd8d5af58f85c60f12c48f8aa880cb50ae4cd57109e9d6c)"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: false,
                 since: "0.8.0".to_string(),
             },
         ]),
        ("sr_sign",
         vec![
             Case {
                 desc: "Use mini secret key".to_string(),
                 input: vec!["-m", "0xc243239f434f7a4b0ab8d4600537001e6479c807c3d3623f99c8ad9f2a588837", "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["0xced639526bb840107f33b7e6588219bae8657707f0537dce9969338748673d54b92e0efba5477a1494696e5cf3f5e7a40f03271b1ef2e2030ef60d6be1caa784"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: false,
                 since: "0.8.0".to_string(),
             },
             Case {
                 desc: "Use secret key".to_string(),
                 input: vec!["-s", "0xb0f4e5710d79bf6a46391e1c6e50a883af767636d55bcad178aa7ec7f1aa750dee6c27bbe26656a29f06ea1612461a86a190db16b31ddd6b78354fb6ba57bf7d", "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["0xced639526bb840107f33b7e6588219bae8657707f0537dce9969338748673d54b92e0efba5477a1494696e5cf3f5e7a40f03271b1ef2e2030ef60d6be1caa784"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: false,
                 since: "0.8.0".to_string(),
             },
         ]),
        ("sr_verify",
         vec![
             Case {
                 desc: "".to_string(),
                 input: vec!["-p", "0x6a8ee649b31efe7aabd8d5af58f85c60f12c48f8aa880cb50ae4cd57109e9d6c", "-S",
                             "0xced639526bb840107f33b7e6588219bae8657707f0537dce9969338748673d54b92e0efba5477a1494696e5cf3f5e7a40f03271b1ef2e2030ef60d6be1caa784",
                             "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["true"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.8.0".to_string(),
             },
         ]),
        ("sr_sk",
         vec![
             Case {
                 desc: "".to_string(),
                 input: vec!["-m", "0xc243239f434f7a4b0ab8d4600537001e6479c807c3d3623f99c8ad9f2a588837"].into_iter().map(Into::into).collect(),
                 output: vec!["0xb0f4e5710d79bf6a46391e1c6e50a883af767636d55bcad178aa7ec7f1aa750dee6c27bbe26656a29f06ea1612461a86a190db16b31ddd6b78354fb6ba57bf7d"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.8.0".to_string(),
             },
         ]),
        ("sr_pk",
         vec![
             Case {
                 desc: "Use mini secret key".to_string(),
                 input: vec!["-m", "0xc243239f434f7a4b0ab8d4600537001e6479c807c3d3623f99c8ad9f2a588837"].into_iter().map(Into::into).collect(),
                 output: vec!["0x6a8ee649b31efe7aabd8d5af58f85c60f12c48f8aa880cb50ae4cd57109e9d6c"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.8.0".to_string(),
             },
             Case {
                 desc: "Use secret key".to_string(),
                 input: vec!["-s", "0xb0f4e5710d79bf6a46391e1c6e50a883af767636d55bcad178aa7ec7f1aa750dee6c27bbe26656a29f06ea1612461a86a190db16b31ddd6b78354fb6ba57bf7d"].into_iter().map(Into::into).collect(),
                 output: vec!["0x6a8ee649b31efe7aabd8d5af58f85c60f12c48f8aa880cb50ae4cd57109e9d6c"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.8.0".to_string(),
             },
         ]),
    ].into_iter().collect()
}
