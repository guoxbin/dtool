use crate::modules::ecdsa::SignatureFormEnum;
use crate::modules::Case;
use linked_hash_map::LinkedHashMap;
use yogcrypt::sm2;
use yogcrypt::sm2::{SecKey, PubKey, Signature};
use std::iter::once;
use std::iter::empty;
use yogcrypt::basic::field::field_p::FieldElement;
use yogcrypt::basic::cell::u64x4::U64x4;

pub fn ec_gk_sm2(compress: bool) -> Result<(Vec<u8>, Vec<u8>), String> {
	if compress == true {
		return Err("Compress is not supported".to_string());
	}

	let secret_key = sm2::get_sec_key();
	let public_key = sm2::get_pub_key(secret_key);

	let secret_key = secret_key_to_vec(secret_key);
	let public_key = public_key_to_vec(public_key);

	Ok((secret_key, public_key))
}

pub fn ec_sign_sm2(secret_key: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<Vec<u8>, String> {
	if let SignatureFormEnum::Der = sig_form {
		return Err("DER form is not supported".to_string());
	}

	let secret_key = vec_to_secret_key(secret_key)?;
	let public_key = sm2::get_pub_key(secret_key);

	let signature = sm2::sm2_gen_sign(&message, secret_key, public_key);

	let signature = signature_to_vec(signature);

	Ok(signature)
}

pub fn ec_verify_sm2(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>, sig_form: SignatureFormEnum) -> Result<(), String> {
	if let SignatureFormEnum::Der = sig_form {
		return Err("DER form is not supported".to_string());
	}

	let public_key = vec_to_public_key(public_key)?;
	let signature = vec_to_signature(sig)?;

	let verified = sm2::sm2_ver_sign(&message, public_key, &signature);

	match verified {
		true => Ok(()),
		false => Err("Invalid signature".to_string()),
	}
}

pub fn ec_pk_sm2(secret_key: Vec<u8>, compress: bool) -> Result<Vec<u8>, String> {
	if compress == true {
		return Err("Compress is not supported".to_string());
	}

	let secret_key = vec_to_secret_key(secret_key)?;
	let public_key = sm2::get_pub_key(secret_key);
	let public_key = public_key_to_vec(public_key);
	Ok(public_key)
}

// Referece: http://www.jonllen.com/upload/jonllen/case/jsrsasign-master/sample-sm2_crypt.html
fn secret_key_to_vec(secret_key: SecKey) -> Vec<u8> {
	let result: Vec<u8> = secret_key.value.iter().rev().map(|x| {
		x.to_be_bytes().to_vec()
	}).flatten().collect();
	result
}

fn public_key_to_vec(public_key: PubKey) -> Vec<u8> {
	let result = once(4u8) // uncompressed
		.chain(public_key.x.num.value.iter().rev().map(|i| {
			i.to_be_bytes().to_vec()
		}).flatten())
		.chain(public_key.y.num.value.iter().rev().map(|i| {
			i.to_be_bytes().to_vec()
		}).flatten()).collect::<Vec<_>>();
	result
}

fn vec_to_secret_key(vec: Vec<u8>) -> Result<SecKey, String> {
	if vec.len() != 32 {
		return Err("Invalid secret key".to_string());
	}

	let secret_key = slice_to_u64x4(&vec);

	Ok(secret_key)
}

fn vec_to_public_key(vec: Vec<u8>) -> Result<PubKey, String> {
	if vec.len() != 65 || vec[0] != 4 {
		return Err("Invalid public key".to_string());
	}

	let x_slice = &vec[1..33];
	let y_slice = &vec[33..65];
	let public_key = PubKey {
		x: FieldElement {
			num: slice_to_u64x4(x_slice)
		},
		y: FieldElement {
			num: slice_to_u64x4(y_slice)
		},
	};

	Ok(public_key)
}

fn signature_to_vec(sig: Signature) -> Vec<u8> {
	let result = empty()
		.chain(sig.r.value.iter().rev().map(|i| {
			i.to_be_bytes().to_vec()
		}).flatten())
		.chain(sig.s.value.iter().rev().map(|i| {
			i.to_be_bytes().to_vec()
		}).flatten()).collect::<Vec<_>>();
	result
}

fn vec_to_signature(vec: Vec<u8>) -> Result<Signature, String> {
	if vec.len() != 64 {
		return Err("Invalid signature".to_string());
	}

	let r_slice = &vec[0..32];
	let s_slice = &vec[32..64];
	let signature = Signature {
		r: slice_to_u64x4(r_slice),
		s: slice_to_u64x4(s_slice),
	};

	Ok(signature)
}

fn slice_to_u64x4(slice: &[u8]) -> U64x4 {
	U64x4 {
		value: [u64::from_be_bytes({ slice_to_arr(&slice[24..32]) }),
			u64::from_be_bytes(slice_to_arr(&slice[16..24])),
			u64::from_be_bytes(slice_to_arr(&slice[8..16])),
			u64::from_be_bytes(slice_to_arr(&slice[0..8]))
		]
	}
}

fn slice_to_arr(slice: &[u8]) -> [u8; 8] {
	let mut a = [0u8; 8];
	a.copy_from_slice(slice);
	a
}

pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
	vec![
		("ec_gk",
		 vec![
			 Case {
				 desc: "SM2".to_string(),
				 input: vec!["-c", "sm2"].into_iter().map(Into::into).collect(),
				 output: vec!["(0x80a61373e34f7215feceb8dd06bb3731ea362ff5355a7226d4e12d076a7eb588, 0x044b2dd8bf6dbbfb14db3e4d17bd7a3e8758eb4232049bec931d1038f4afaae46ac3c771f929bbf35a28b0363789fb19127cea3318f4c8902a0034ca5f1b7667d1)"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
		 ]),
		("ec_sign",
		 vec![
			 Case {
				 desc: "SM2".to_string(),
				 input: vec!["-c", "sm2", "-s", "0x80a61373e34f7215feceb8dd06bb3731ea362ff5355a7226d4e12d076a7eb588", "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["0x0a4d089d3177234ed34aa7f30c6a7a7954539f68825bedbe82be65aefdb733c921207be31b8071bbfd5c99044ebde49d3c38e9972063b844f65f4acfc7d6dff2"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: false,
				 since: "0.7.0".to_string(),
			 },
		 ]),
		("ec_verify",
		 vec![
			 Case {
				 desc: "SM2".to_string(),
				 input: vec!["-c", "sm2", "-p", "0x044b2dd8bf6dbbfb14db3e4d17bd7a3e8758eb4232049bec931d1038f4afaae46ac3c771f929bbf35a28b0363789fb19127cea3318f4c8902a0034ca5f1b7667d1", "-S",
				             "0x0a4d089d3177234ed34aa7f30c6a7a7954539f68825bedbe82be65aefdb733c921207be31b8071bbfd5c99044ebde49d3c38e9972063b844f65f4acfc7d6dff2",
				             "0x616263"].into_iter().map(Into::into).collect(),
				 output: vec!["true"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: true,
				 since: "0.7.0".to_string(),
			 },
		 ]),
		("ec_pk",
		 vec![
			 Case {
				 desc: "SM2".to_string(),
				 input: vec!["-c", "sm2", "-s", "0x80a61373e34f7215feceb8dd06bb3731ea362ff5355a7226d4e12d076a7eb588"].into_iter().map(Into::into).collect(),
				 output: vec!["0x044b2dd8bf6dbbfb14db3e4d17bd7a3e8758eb4232049bec931d1038f4afaae46ac3c771f929bbf35a28b0363789fb19127cea3318f4c8902a0034ca5f1b7667d1"].into_iter().map(Into::into).collect(),
				 is_example: true,
				 is_test: true,
				 since: "0.7.0".to_string(),
			 },
		 ]),
	].into_iter().collect()
}
