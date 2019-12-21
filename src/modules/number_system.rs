use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ns").about("Number system")
				.arg(
					Arg::with_name("d")
						.short("d").help("Output decimal result")
						.required(false))
				.arg(
					Arg::with_name("b")
						.short("b").help("Output binary result")
						.required(false))
				.arg(
					Arg::with_name("o")
						.short("o").help("Output octal result")
						.required(false))
				.arg(
					Arg::with_name("x")
						.short("x").help("Output hexadecimal result")
						.required(false))
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: ns,
		},
	]
}

fn ns(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let (radix, number) = match input {
		_ if input.starts_with("0b") => (2, &input[2..]),
		_ if input.starts_with("0o") => (8, &input[2..]),
		_ if input.starts_with("0x") => (16, &input[2..]),
		_ => (10, &input[..]),
	};

	let number = u64::from_str_radix(number, radix).map_err(|_| "Invalid input")?;

	let mut results = Vec::new();

	if matches.is_present("d") {
		results.push(format!("{}", number));
	}
	if matches.is_present("b") {
		results.push(format!("0b{:b}", number));
	}
	if matches.is_present("o") {
		results.push(format!("0o{:o}", number));
	}
	if matches.is_present("x") {
		results.push(format!("0x{:x}", number));
	}
	if results.len() == 0 {
		results = vec![
			format!("{}", number),
			format!("0b{:b}", number),
			format!("0o{:o}", number),
			format!("0x{:x}", number),
		];
	}

	Ok(results)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_ns() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["ns", "256"]);
		assert_eq!(ns(&matches) , Ok(vec!["256".to_string(), "0b100000000".to_string(), "0o400".to_string(), "0x100".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ns", "-d", "256"]);
		assert_eq!(ns(&matches) , Ok(vec!["256".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ns", "-b", "256"]);
		assert_eq!(ns(&matches) , Ok(vec!["0b100000000".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ns", "-o", "256"]);
		assert_eq!(ns(&matches) , Ok(vec!["0o400".to_string()]));

		let matches = app.clone().get_matches_from(vec!["ns", "-x", "256"]);
		assert_eq!(ns(&matches) , Ok(vec!["0x100".to_string()]));

	}

}
