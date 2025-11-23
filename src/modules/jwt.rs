use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "JWT (JSON Web Token) tools".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("jwt_decode")
				.about("Decode JWT token (without verification)")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: jwt_decode,
		},
		Command {
			app: SubCommand::with_name("jwt_encode")
				.about("Encode JWT token")
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)
						.help("JSON payload"),
				)
				.arg(
					Arg::with_name("algorithm")
						.short("a")
						.long("algorithm")
						.takes_value(true)
						.default_value("HS256")
						.help("Algorithm: HS256, HS384, HS512"),
				)
				.arg(
					Arg::with_name("secret")
						.short("s")
						.long("secret")
						.takes_value(true)
						.required(true)
						.help("Secret key"),
				)
				.arg(
					Arg::with_name("exp")
						.short("e")
						.long("exp")
						.takes_value(true)
						.help("Expiration time in seconds from now"),
				),
			f: jwt_encode,
		},
		Command {
			app: SubCommand::with_name("jwt_verify")
				.about("Verify JWT token")
				.arg(Arg::with_name("INPUT").required(false).index(1))
				.arg(
					Arg::with_name("algorithm")
						.short("a")
						.long("algorithm")
						.takes_value(true)
						.default_value("HS256")
						.help("Algorithm: HS256, HS384, HS512"),
				)
				.arg(
					Arg::with_name("secret")
						.short("s")
						.long("secret")
						.takes_value(true)
						.required(true)
						.help("Secret key"),
				),
			f: jwt_verify,
		},
	]
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	#[serde(flatten)]
	data: Value,
}

fn parse_algorithm(alg: &str) -> Result<Algorithm, String> {
	match alg.to_uppercase().as_str() {
		"HS256" => Ok(Algorithm::HS256),
		"HS384" => Ok(Algorithm::HS384),
		"HS512" => Ok(Algorithm::HS512),
		_ => Err(format!("Unsupported algorithm: {}", alg)),
	}
}

fn jwt_decode(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;
	let token = input.trim();

	// Decode header
	let header = jsonwebtoken::decode_header(token).map_err(|e| format!("Invalid JWT: {}", e))?;

	// Decode payload without verification
	let mut validation = Validation::default();
	validation.insecure_disable_signature_validation();
	validation.validate_exp = false;
	validation.required_spec_claims.clear(); // Don't require any claims

	let token_data = decode::<Value>(token, &DecodingKey::from_secret(&[]), &validation)
		.map_err(|e| format!("Failed to decode JWT: {}", e))?;

	let header_json = serde_json::to_string_pretty(&header).map_err(|e| e.to_string())?;
	let payload_json =
		serde_json::to_string_pretty(&token_data.claims).map_err(|e| e.to_string())?;

	Ok(vec![
		format!("Header: {}", header_json),
		format!("Payload: {}", payload_json),
	])
}

fn jwt_encode(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;
	let secret = matches.value_of("secret").unwrap();
	let alg_str = matches.value_of("algorithm").unwrap();
	let algorithm = parse_algorithm(alg_str)?;

	// Parse the input JSON
	let mut payload: Value = serde_json::from_str(&input)
		.map_err(|e| format!("Invalid JSON payload: {}", e))?;

	// Add expiration if specified
	if let Some(exp_str) = matches.value_of("exp") {
		let exp_seconds: u64 = exp_str.parse().map_err(|_| "Invalid expiration time")?;
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|e| e.to_string())?
			.as_secs();
		let exp = now + exp_seconds;

		if let Value::Object(ref mut map) = payload {
			map.insert("exp".to_string(), json!(exp));
		}
	}

	let claims = Claims { data: payload };

	let header = Header::new(algorithm);
	let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
		.map_err(|e| format!("Failed to encode JWT: {}", e))?;

	Ok(vec![token])
}

fn jwt_verify(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;
	let token = input.trim();
	let secret = matches.value_of("secret").unwrap();
	let alg_str = matches.value_of("algorithm").unwrap();
	let algorithm = parse_algorithm(alg_str)?;

	let mut validation = Validation::new(algorithm);
	validation.validate_exp = true;
	validation.required_spec_claims.clear(); // Don't require exp claim

	match decode::<Value>(
		token,
		&DecodingKey::from_secret(secret.as_bytes()),
		&validation,
	) {
		Ok(token_data) => {
			let payload_json =
				serde_json::to_string_pretty(&token_data.claims).map_err(|e| e.to_string())?;
			Ok(vec![
				"Valid: true".to_string(),
				format!("Payload: {}", payload_json),
			])
		}
		Err(e) => Ok(vec![
			"Valid: false".to_string(),
			format!("Error: {:?}", e.kind()),
		]),
	}
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			(
				"jwt_decode",
				vec![Case {
					desc: "Decode JWT token".to_string(),
					input: vec!["eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"]
						.into_iter()
						.map(Into::into)
						.collect(),
					output: vec![
					r#"Header: {
  "typ": "JWT",
  "alg": "HS256"
}"#,
					r#"Payload: {
  "iat": 1516239022,
  "name": "John Doe",
  "sub": "1234567890"
}"#,
				]
					.into_iter()
					.map(Into::into)
					.collect(),
					is_example: true,
					is_test: true,
					since: "0.16.0".to_string(),
				}],
			),
			(
				"jwt_encode",
				vec![Case {
					desc: "Encode JWT token with HS256".to_string(),
					input: vec!["-a", "HS256", "-s", "your-256-bit-secret", r#"{"sub":"1234567890","name":"John Doe","iat":1516239022}"#]
						.into_iter()
						.map(Into::into)
						.collect(),
					output: vec!["eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1MTYyMzkwMjIsIm5hbWUiOiJKb2huIERvZSIsInN1YiI6IjEyMzQ1Njc4OTAifQ.AcEGBRSPsmXJUzuoLfM-z2JTumSEjCNAW4Fzjl0wVgM"]
						.into_iter()
						.map(Into::into)
						.collect(),
					is_example: true,
					is_test: true,
					since: "0.16.0".to_string(),
				}],
			),
			(
				"jwt_verify",
				vec![Case {
					desc: "Verify JWT token with correct secret".to_string(),
					input: vec!["-a", "HS256", "-s", "your-256-bit-secret", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1MTYyMzkwMjIsIm5hbWUiOiJKb2huIERvZSIsInN1YiI6IjEyMzQ1Njc4OTAifQ.AcEGBRSPsmXJUzuoLfM-z2JTumSEjCNAW4Fzjl0wVgM"]
						.into_iter()
						.map(Into::into)
						.collect(),
					output: vec![
						"Valid: true",
						r#"Payload: {
  "iat": 1516239022,
  "name": "John Doe",
  "sub": "1234567890"
}"#,
					]
					.into_iter()
					.map(Into::into)
					.collect(),
					is_example: true,
					is_test: true,
					since: "0.16.0".to_string(),
				}],
			),
		]
		.into_iter()
		.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::base::test::test_module;

	#[test]
	fn test_cases() {
		test_module(module());
	}
}
