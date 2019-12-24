use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base};
use escaper;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("he").about("HTML entity encode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: he,
		},
		Command {
			app: SubCommand::with_name("hd").about("HTML entity decode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: hd,
		}
	]
}

fn he(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = escaper::encode_minimal(&input);

	Ok(vec![result])
}

fn hd(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = base::input_string(matches)?;

	let result = escaper::decode_html(&input).map_err(|_| "Decode failed")?;

	Ok(vec![result])
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_he() {
		let app =  &commands()[0].app;

		let matches = app.clone().get_matches_from(vec!["he", "<br>"]);
		assert_eq!(he(&matches) , Ok(vec!["&lt;br&gt;".to_string()]));
	}

	#[test]
	fn test_hd() {
		let app =  &commands()[1].app;

		let matches = app.clone().get_matches_from(vec!["hd", "&lt;br&gt;"]);
		assert_eq!(hd(&matches) , Ok(vec!["<br>".to_string()]));

	}

}
