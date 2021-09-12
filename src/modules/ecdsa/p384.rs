use crate::modules::ecdsa::SignatureFormEnum;
use crate::modules::Case;
use linked_hash_map::LinkedHashMap;
use ring::signature::{VerificationAlgorithm, ECDSA_P384_SHA384_ASN1, ECDSA_P384_SHA384_FIXED};
use untrusted::Input;

pub fn ec_gk_p384(_compress: bool) -> Result<(Vec<u8>, Vec<u8>), String> {
	unimplemented!()
}

pub fn ec_sign_p384(
	_secret_key: Vec<u8>,
	_message: Vec<u8>,
	_sig_form: SignatureFormEnum,
) -> Result<Vec<u8>, String> {
	unimplemented!()
}

pub fn ec_verify_p384(
	public_key: Vec<u8>,
	sig: Vec<u8>,
	message: Vec<u8>,
	sig_form: SignatureFormEnum,
) -> Result<(), String> {
	let algo = match sig_form {
		SignatureFormEnum::Fixed => &ECDSA_P384_SHA384_FIXED,
		SignatureFormEnum::Der => &ECDSA_P384_SHA384_ASN1,
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

pub fn ec_pk_p384(_secret_key: Vec<u8>, _compress: bool) -> Result<Vec<u8>, String> {
	unimplemented!()
}

pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
	vec![
		("ec_gk",
		 vec![
			 Case {
				 desc: "P-384".to_string(),
				 input: vec!["-c", "p384"].into_iter().map(Into::into).collect(),
				 output: vec!["(0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f, 0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3)"].into_iter().map(Into::into).collect(),
				 is_example: false,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
		 ]),
		("ec_sign",
		 vec![
			 Case {
				 desc: "P-384".to_string(),
				 input: vec!["-c", "p384", "-s", "0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f", "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["0xa0d387bc5d5de4979750f531f337fd1d04384ab4a9d251a18852c1ce1a16e2e46a2778764d0b3ee090babbc5092ea57a108ddabf9a9fcf8efaad7c0862da2beddde806745c0c3972d738c416d55cfde19b85e39ab54151c87b537c4df7d177ff"].into_iter().map(Into::into).collect(),
				 is_example: false,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
			 Case {
				 desc: "P-384 DER signature form".to_string(),
				 input: vec!["-c", "p384", "-s", "0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f", "-f", "der", "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["0x3065023100e48b9cd154ecd8dfd138f2e3c5d79af62b3cdc413e52565822edcc96786b03d8e996f132cf793b17c267dc177a5e6525023043dd0485f762b48e3a4a9daeeef57ceff2cf84da6a00b6a65293ee7233efe392ba4514a475476815dddfbbb7ea9e269c"].into_iter().map(Into::into).collect(),
				 is_example: false,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
		 ]),
		("ec_verify",
		 vec![
			 Case {
				 desc: "P-384".to_string(),
				 input: vec!["-c", "p384", "-p", "0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3", "-S",
				             "0xa0d387bc5d5de4979750f531f337fd1d04384ab4a9d251a18852c1ce1a16e2e46a2778764d0b3ee090babbc5092ea57a108ddabf9a9fcf8efaad7c0862da2beddde806745c0c3972d738c416d55cfde19b85e39ab54151c87b537c4df7d177ff",
				             "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["true"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: true,
				 since: "0.7.0".to_string(),
			 },
			 Case {
				 desc: "P-384 DER signature form".to_string(),
				 input: vec!["-c", "p384", "-p", "0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3", "-f", "der", "-S",
				             "0x3065023100e48b9cd154ecd8dfd138f2e3c5d79af62b3cdc413e52565822edcc96786b03d8e996f132cf793b17c267dc177a5e6525023043dd0485f762b48e3a4a9daeeef57ceff2cf84da6a00b6a65293ee7233efe392ba4514a475476815dddfbbb7ea9e269c",
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
				 desc: "P-384".to_string(),
				 input: vec!["-c", "p384", "-s", "0xfbc89e8fae9340747f162330345f7cfac7387a2049f6bedb55f7a30faf8b1d24da9b1e618db7b215daa1c7b0fd54858f"].into_iter().map(Into::into).collect(),
				 output: vec!["0x044978c6c7be1a5c5194983a945d2d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3"].into_iter().map(Into::into).collect(),
				 is_example: false,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
		 ]),
	].into_iter().collect()
}
