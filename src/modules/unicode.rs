use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
use std::char::EscapeUnicode;

static FORMAT_HELP: &str = "Format:
<default>: \\u7c
html: &#x7c;
html_d: &#124;
rust: \\u{7c}";

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2u").about("")
				.arg(
					Arg::with_name("f")
						.short("f").help(FORMAT_HELP)
						.takes_value(true)
						.required(false))
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: h2u,
		},
		Command {
			app: SubCommand::with_name("u2h").about("")
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: u2h,
		},
	]
}

fn h2u(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let input = String::from_utf8(input).map_err(|_| "Convert failed")?;

	let format = match matches.value_of("f") {
		Some("html") => format_html,
		Some("html_d") => format_html_d,
		Some("rust") => format_rust,
		_ => format_default,
	};

	let result = input.chars().map(char::escape_unicode).map(format).collect::<Result<Vec<String>, String>>()?;

	let result = result.join("");

	Ok(vec![result])
}

fn u2h(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let format = match input {
		_ if input.starts_with("\\u{") => "rust",
		_ if input.starts_with("&#x") => "html",
		_ if input.starts_with("&#") => "html_d",
		_ => "",
	};

	let result = match format {
		"html" => {
			input.split(";")
				.filter_map(from_html).collect::<Result<String, String>>()
		}
		"html_d" => {
			input.split(";")
				.filter_map(from_html_d).collect::<Result<String, String>>()
		}
		"rust" => {
			input.split("}")
				.filter_map(from_rust).collect::<Result<String, String>>()
		}
		_ => {
			input.split("\\u")
				.filter_map(from_default).collect::<Result<String, String>>()
		}
	}?;

	let result = hex::encode(result);
	let result = "0x".to_string() + &result;

	Ok(vec![result])
}

fn format_html(data: EscapeUnicode) -> Result<String, String> {
	Ok(data.map(|x| {
		match x {
			'\\' => '&',
			'u' => '#',
			'{' => 'x',
			'}' => ';',
			_ => x,
		}
	}).collect())
}

fn from_html(data: &str) -> Option<Result<char, String>> {
	if data.len() > 3 {
		let r = u32::from_str_radix(&data[3..], 16).map_err(|_| "Convert failed".to_string()).and_then(|x| {
			std::char::from_u32(x).ok_or("Convert failed".to_string())
		});
		Some(r)
	} else {
		None
	}
}

fn format_html_d(data: EscapeUnicode) -> Result<String, String> {
	let number = data.filter_map(|x| match x {
		'\\' | 'u' | '{' | '}' => None,
		_ => Some(x),
	}).collect::<String>();
	let number = u64::from_str_radix(&number, 16).map_err(|_| "Convert failed")?;

	Ok(format!("&#{};", number))
}

fn from_html_d(data: &str) -> Option<Result<char, String>> {
	if data.len() > 2 {
		let r = u32::from_str_radix(&data[2..], 10).map_err(|_| "Convert failed".to_string()).and_then(|x| {
			std::char::from_u32(x).ok_or("Convert failed".to_string())
		});
		Some(r)
	} else {
		None
	}
}

fn format_rust(data: EscapeUnicode) -> Result<String, String> {
	Ok(data.collect())
}

fn from_rust(data: &str) -> Option<Result<char, String>> {
	if data.len() > 3 {
		let r = u32::from_str_radix(&data[3..], 16).map_err(|_| "Convert failed".to_string()).and_then(|x| {
			std::char::from_u32(x).ok_or("Convert failed".to_string())
		});
		Some(r)
	} else {
		None
	}
}

fn format_default(data: EscapeUnicode) -> Result<String, String> {
	Ok(data.filter(|x| x != &'{' && x != &'}').collect())
}

fn from_default(data: &str) -> Option<Result<char, String>> {
	if data.len() > 0 {
		let r = u32::from_str_radix(&data, 16).map_err(|_| "Convert failed".to_string()).and_then(|x| {
			std::char::from_u32(x).ok_or("Convert failed".to_string())
		});
		Some(r)
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_h2u() {
		let app = &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["h2u", "0x616263"]);
		assert_eq!(h2u(&matches), Ok(vec!["\\u61\\u62\\u63".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2u", "-f", "html", "0x616263"]);
		assert_eq!(h2u(&matches), Ok(vec!["&#x61;&#x62;&#x63;".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2u", "-f", "html_d", "0x616263"]);
		assert_eq!(h2u(&matches), Ok(vec!["&#97;&#98;&#99;".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2u", "-f", "rust", "0x616263"]);
		assert_eq!(h2u(&matches), Ok(vec!["\\u{61}\\u{62}\\u{63}".to_string()]));
	}

	#[test]
	fn test_u2h() {
		let app = &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["h2u", "\\u61\\u62\\u63"]);
		assert_eq!(u2h(&matches), Ok(vec!["0x616263".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2u", "&#x61;&#x62;&#x63;"]);
		assert_eq!(u2h(&matches), Ok(vec!["0x616263".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2u", "&#97;&#98;&#99;"]);
		assert_eq!(u2h(&matches), Ok(vec!["0x616263".to_string()]));

		let matches = app.clone().get_matches_from(vec!["h2u", "\\u{61}\\u{62}\\u{63}"]);
		assert_eq!(u2h(&matches), Ok(vec!["0x616263".to_string()]));
	}
}
