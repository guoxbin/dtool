use crate::app;
use clap::{App, Arg, ArgMatches, Shell, SubCommand};
use std::io::stdout;
use std::str::FromStr;

pub fn app<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name("completion")
		.about("Generate completion")
		.arg(
			Arg::with_name("SHELL")
				.long("shell")
				.short("s")
				.help("Shell")
				.takes_value(true)
				.possible_values(&Shell::variants())
				.required(true),
		)
}

pub fn run(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let shell = matches
		.value_of("SHELL")
		.ok_or_else(|| "Invalid shell".to_string())?;
	let shell = Shell::from_str(shell)?;

	let (mut app, _) = app::build_app();
	app.gen_completions_to("dtool", shell, &mut stdout());

	Ok(vec![])
}
