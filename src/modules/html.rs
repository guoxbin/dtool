use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Case};
use escaper;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("he").about("HTML entity encode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: he,
			cases: vec![
				Case {
					input: vec!["<b>"].into_iter().map(Into::into).collect(),
					output: vec!["&lt;b&gt;"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
		},
		Command {
			app: SubCommand::with_name("hd").about("HTML entity decode").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: hd,
			cases: vec![
				Case {
					input: vec!["&lt;b&gt;"].into_iter().map(Into::into).collect(),
					output: vec!["<b>"].into_iter().map(Into::into).collect(),
					is_example: true,
					is_test: true,
				},
			],
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
	use crate::modules::base::test::test_commands;

	#[test]
	fn test_cases() {
		test_commands(&commands());
	}

}
