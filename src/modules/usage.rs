use clap::{SubCommand, Arg, App, ArgMatches};
use crate::modules::Command;
use linked_hash_map::LinkedHashMap;
use prettytable::{Table, Row, Cell, format};
use std::cmp::min;
use madato::mk_table;
use escaper;
use regex::Captures;

pub fn usage_app<'a, 'b>() -> App<'a, 'b> {

	SubCommand::with_name("usage").about("Show usage")
		.arg(
		Arg::with_name("FORMAT")
			.long("format")
			.short("f").help("Output format\n<default>: term table format\nmarkdown: markdown format")
			.takes_value(true)
			.required(false))
		.arg(
		Arg::with_name("SEARCH")
			.long("search")
			.short("s").help("")
			.takes_value(true)
			.required(false))
}

pub fn usage<'a, 'b>(matches: &ArgMatches, commands: &LinkedHashMap<String, Command<'a, 'b>>) -> Result<Vec<String>, String> {

	let table = get_usage_table(commands);

	let search = matches.value_of("SEARCH");

	let table = match search {
		Some(search) => {
			let search = &search.to_lowercase();
			let (header, body) = table;
			let body = body.into_iter().filter(|row| {
				row.iter().any(|cell|cell.to_lowercase().contains(search))
			}).collect();
			(header, body)
		},
		_ => table,
	};

	let format = matches.value_of("FORMAT");

	match format{
		Some("markdown") => markdown_output(table),
		_ => term_output(table),
	}
}

fn term_output(table: (Vec<String>, Vec<Vec<String>>)) -> Result<Vec<String>, String> {

	let (header, body) = table;

	let mut result = vec![
		"Usage".to_string(),
		"".to_string()
	];

	if body.len() == 0 {
		return Ok(result);
	}

	let mut table = Table::init(body.iter().map(|row| Row::new(row.iter().map(|c|Cell::new(c)).collect()) ).collect());
	table.set_titles(Row::new(header.iter().map(|c|Cell::new(c)).collect()) );
	table.set_format(*format::consts::FORMAT_NO_COLSEP);

	result.push(table.to_string());

	Ok(result)
}

fn markdown_output(table: (Vec<String>, Vec<Vec<String>>)) -> Result<Vec<String>, String> {

	let (header, body) = table;

	let mut result = vec![
		"# Usage".to_string(),
		"".to_string()
	];

	if body.len() == 0 {
		return Ok(result);
	}

	let blanks_re = regex::Regex::new(" {2,}").expect("qed");

	let cell_process = |cell: String| {
		let cell = escaper::encode_minimal(&cell);
		// blank(more then 1) to &nbsp;
		let cell = blanks_re.replace_all(&cell, |caps: &Captures|{
			caps[0].replace(" ", "&nbsp;")
		});
		// enter to <br>
		cell.replace("\n", "<br>")
	};

	let body : Vec<LinkedHashMap<String, String>> = body.into_iter().map(|row|{
		row.into_iter().enumerate().map(|(col, cell)|( header[col].clone(), cell_process(cell))).collect::<LinkedHashMap<String, String>>()
	}).collect();

	let table = mk_table(&body, &None);

	result.push(table);

	Ok(result)
}

/// Get usage table
/// Sub command, Desc, Example, Remark, Since
fn get_usage_table(commands: &LinkedHashMap<String, Command>) -> (Vec<String>, Vec<Vec<String>>) {

	let header = vec!["Sub command", "Desc", "Example", "Remark", "Since"].into_iter().map(Into::into).collect();

	let mut body = vec![];
	for (name, command) in commands {
		let about = get_about(&command.app);
		for case in &command.cases {
			if case.is_example {
				let input = case.input.clone();
				let output = case.output.clone();

				let input = vec!["$ dtool".to_string(), name.clone()].into_iter().chain(input).collect::<Vec<String>>().join(" ");
				let example = vec![input].into_iter().chain(output)
					.map(|x: String|{
						add_enter(x, 50)
					})
					.collect::<Vec<String>>().join("\n");

				let since = format!("v{}", case.since);
				let item = vec![name.to_owned(), about.clone(), example, case.desc.clone(), since];
				body.push(item);
			}
		}

	}
	(header, body)
}

fn get_about(app: &App) -> String {

	let mut help = Vec::new();
	let _ = app.write_help(&mut help);

	let about = String::from_utf8(help)
		.map(|x|x.split("\n").skip(1).take(1).collect::<String>());//about is the second line

	about.ok().unwrap_or_default()
}

fn add_enter(data: String, len: usize) -> String {

	let mut v = vec![];
	let mut cur = data.as_str();
	while !cur.is_empty() {
		let (chunk, rest) = cur.split_at(min(len, cur.len()));
		v.push(chunk);
		cur = rest;
	}
	v.join("\\ \n")

}