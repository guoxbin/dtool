use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};
use uuid::{Timestamp, Uuid};

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "UUID generation and parsing".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("uuid_gen")
				.about("Generate UUID")
				.arg(
					Arg::with_name("version")
						.short("v")
						.long("version")
						.takes_value(true)
						.default_value("4")
						.help("UUID version: 1, 4, 5, 7"),
				)
				.arg(
					Arg::with_name("namespace")
						.short("n")
						.long("namespace")
						.takes_value(true)
						.help("Namespace for v5: dns, url, oid, x500"),
				)
				.arg(
					Arg::with_name("name")
						.short("s")
						.long("name")
						.takes_value(true)
						.help("Name for v5 UUID"),
				),
			f: uuid_gen,
		},
		Command {
			app: SubCommand::with_name("uuid_parse")
				.about("Parse UUID and show details")
				.arg(Arg::with_name("INPUT").required(false).index(1)),
			f: uuid_parse,
		},
	]
}

fn uuid_gen(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let version = matches.value_of("version").unwrap();

	let uuid = match version {
		"1" => {
			// Generate v1 UUID (timestamp-based)
			let ts = Timestamp::now(uuid::timestamp::context::NoContext);
			Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6])
		}
		"4" => {
			// Generate v4 UUID (random)
			Uuid::new_v4()
		}
		"5" => {
			// Generate v5 UUID (namespace + name)
			let namespace_str = matches
				.value_of("namespace")
				.ok_or("Namespace (-n) is required for v5")?;
			let name = matches
				.value_of("name")
				.ok_or("Name (-s) is required for v5")?;

			let namespace = match namespace_str.to_lowercase().as_str() {
				"dns" => uuid::Uuid::NAMESPACE_DNS,
				"url" => uuid::Uuid::NAMESPACE_URL,
				"oid" => uuid::Uuid::NAMESPACE_OID,
				"x500" => uuid::Uuid::NAMESPACE_X500,
				_ => {
					return Err(format!(
						"Invalid namespace: {}. Use dns, url, oid, or x500",
						namespace_str
					))
				}
			};

			Uuid::new_v5(&namespace, name.as_bytes())
		}
		"7" => {
			// Generate v7 UUID (timestamp-based, sortable)
			let ts = Timestamp::now(uuid::timestamp::context::NoContext);
			Uuid::new_v7(ts)
		}
		_ => return Err(format!("Unsupported UUID version: {}", version)),
	};

	Ok(vec![uuid.to_string()])
}

fn uuid_parse(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;
	let uuid_str = input.trim();

	let uuid = Uuid::parse_str(uuid_str).map_err(|e| format!("Invalid UUID: {}", e))?;

	let mut result = Vec::new();

	// Version
	result.push(format!("Version: {}", get_version_name(uuid.get_version_num())));

	// Variant
	result.push(format!("Variant: {}", get_variant_name(&uuid)));

	// Timestamp (for v1 and v7)
	if let Some(version) = uuid.get_version() {
		match version {
			uuid::Version::Mac => {
				if let Some(ts) = uuid.get_timestamp() {
					let (secs, nanos) = ts.to_unix();
					result.push(format!("Timestamp: {} seconds, {} nanoseconds", secs, nanos));
				}
			}
			uuid::Version::SortRand => {
				if let Some(ts) = uuid.get_timestamp() {
					let (secs, nanos) = ts.to_unix();
					result.push(format!("Timestamp: {} seconds, {} nanoseconds", secs, nanos));
				}
			}
			_ => {}
		}
	}

	result.push(format!("Valid: true"));

	Ok(result)
}

fn get_version_name(version: usize) -> String {
	match version {
		1 => "1 (Timestamp and MAC)".to_string(),
		2 => "2 (DCE Security)".to_string(),
		3 => "3 (MD5 hash)".to_string(),
		4 => "4 (Random)".to_string(),
		5 => "5 (SHA-1 hash)".to_string(),
		6 => "6 (Reordered timestamp)".to_string(),
		7 => "7 (Unix timestamp)".to_string(),
		8 => "8 (Custom)".to_string(),
		_ => format!("{} (Unknown)", version),
	}
}

fn get_variant_name(uuid: &Uuid) -> String {
	match uuid.get_variant() {
		uuid::Variant::NCS => "NCS".to_string(),
		uuid::Variant::RFC4122 => "RFC 4122".to_string(),
		uuid::Variant::Microsoft => "Microsoft".to_string(),
		uuid::Variant::Future => "Future".to_string(),
		_ => "Unknown".to_string(),
	}
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			(
				"uuid_gen",
				vec![
					Case {
						desc: "Generate UUID v4 (random)".to_string(),
						input: vec![].into_iter().map(|s: &str| s.to_string()).collect(),
						output: vec![] // UUID is random, can't predict output
							.into_iter()
							.map(|s: &str| s.to_string())
							.collect(),
						is_example: true,
						is_test: false, // Skip test for random UUID
						since: "0.16.0".to_string(),
					},
					Case {
						desc: "Generate UUID v5 with DNS namespace".to_string(),
						input: vec!["-v", "5", "-n", "dns", "-s", "example.com"]
							.into_iter()
							.map(Into::into)
							.collect(),
						output: vec!["cfbff0d1-9375-5685-968c-48ce8b15ae17"]
							.into_iter()
							.map(Into::into)
							.collect(),
						is_example: true,
						is_test: true,
						since: "0.16.0".to_string(),
					},
				],
			),
			(
				"uuid_parse",
				vec![Case {
					desc: "Parse UUID v4".to_string(),
					input: vec!["550e8400-e29b-41d4-a716-446655440000"]
						.into_iter()
						.map(Into::into)
						.collect(),
					output: vec![
						"Version: 4 (Random)",
						"Variant: RFC 4122",
						"Valid: true",
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
