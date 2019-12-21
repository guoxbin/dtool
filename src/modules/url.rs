use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
use urlencoding;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("ue").about("URL encode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ue,
		},
		Command {
			app: SubCommand::with_name("ud").about("URL decode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: ud,
		}
	]
}

fn ue(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = urlencoding::encode(&input);

	Ok(vec![result])
}

fn ud(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = urlencoding::decode(&input).map_err(|_| "Decode failed")?;

	Ok(vec![result])
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_ue() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["ue", "a+b"]);
		assert_eq!(ue(&matches) , Ok(vec!["a%2Bb".to_string()]));
	}

	#[test]
	fn test_ud() {
		let app =  &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["ud", "a%2Bb"]);
		assert_eq!(ud(&matches) , Ok(vec!["a+b".to_string()]));

	}

}
