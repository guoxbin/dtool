use secp256k1::rand::{thread_rng, Rng};
use crate::modules::Case;
use linked_hash_map::LinkedHashMap;
use ring::signature::{Ed25519KeyPair, KeyPair, ED25519, VerificationAlgorithm};
use untrusted::Input;

pub fn ed_gk_ed25519() -> Result<(Vec<u8>, Vec<u8>), String> {

    let seed = random_32_bytes(&mut thread_rng());

    let key_pair = Ed25519KeyPair::from_seed_unchecked(&seed).map_err(|_| "Invalid secret key")?;

    let secret_key = seed.to_vec();

    let public_key = key_pair.public_key().as_ref().to_vec();

    Ok((secret_key, public_key))
}

pub fn ed_sign_ed25519(secret_key: Vec<u8>, message: Vec<u8>) -> Result<Vec<u8>, String> {

    let key_pair = Ed25519KeyPair::from_seed_unchecked(&secret_key).map_err(|_| "Invalid secret key")?;

    let signature =  key_pair.sign(&message);

    let signature = signature.as_ref().to_vec();

    Ok(signature)
}

pub fn ed_verify_ed25519(public_key: Vec<u8>, sig: Vec<u8>, message: Vec<u8>) -> Result<(), String> {

    let result = ED25519.verify(Input::from(&public_key),
                             Input::from(&message), Input::from(&sig))
        .map_err(|e| format!("Invalid signature: {}", e))?;

    Ok(result)
}

pub fn ed_pk_ed25519(secret_key: Vec<u8>) -> Result<Vec<u8>, String> {

    let key_pair = Ed25519KeyPair::from_seed_unchecked(&secret_key).map_err(|_| "Invalid secret key")?;

    let public_key = key_pair.public_key();
    let public_key = public_key.as_ref().to_vec();

    Ok(public_key)
}

fn random_32_bytes<R: Rng + ?Sized>(rng: &mut R) -> [u8; 32] {
    let mut ret = [0u8; 32];
    rng.fill_bytes(&mut ret);
    ret
}

pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
    vec![
        ("ed_gk",
         vec![
             Case {
                 desc: "".to_string(),
                 input: Vec::<String>::new().into_iter().map(Into::into).collect(),
                 output: vec!["(0xb850164d1feec8698acca329947c9885bd1d94034d2fbbe6080598adbe15b298, 0x892c89a4cd631d08da314607223814775604535a05f50e959d21209d01740eba)"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: false,
                 since: "0.8.0".to_string(),
             },
         ]),
        ("ed_sign",
         vec![
             Case {
                 desc: "".to_string(),
                 input: vec!["-s", "0xb850164d1feec8698acca329947c9885bd1d94034d2fbbe6080598adbe15b298", "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["0x52131a69ebb236703de0c3589689202eebd1d16c40990c3ad8b3582631a7a267db745dbb9156d8626187e40f42f6cfe884b6d3ce0cdc04603afeed089703ac0e"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.8.0".to_string(),
             },
         ]),
        ("ed_verify",
         vec![
             Case {
                 desc: "".to_string(),
                 input: vec!["-p", "0x892c89a4cd631d08da314607223814775604535a05f50e959d21209d01740eba", "-S",
                             "0x52131a69ebb236703de0c3589689202eebd1d16c40990c3ad8b3582631a7a267db745dbb9156d8626187e40f42f6cfe884b6d3ce0cdc04603afeed089703ac0e",
                             "0x616263"].into_iter().map(Into::into).collect(),
                 output: vec!["true"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.8.0".to_string(),
             },
         ]),
        ("ed_pk",
         vec![
             Case {
                 desc: "".to_string(),
                 input: vec!["-s", "0xb850164d1feec8698acca329947c9885bd1d94034d2fbbe6080598adbe15b298"].into_iter().map(Into::into).collect(),
                 output: vec!["0x892c89a4cd631d08da314607223814775604535a05f50e959d21209d01740eba"].into_iter().map(Into::into).collect(),
                 is_example: true,
                 is_test: true,
                 since: "0.8.0".to_string(),
             },
         ]),
    ].into_iter().collect()
}
