use clap::{SubCommand, Arg, App, ArgMatches};
use crate::modules::Module;
use prettytable::{Table, Row, Cell, format};
use std::cmp::min;
use madato::mk_table;
use escaper;
use regex::Captures;
use linked_hash_map::LinkedHashMap;

pub fn app<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name("usage").about("Show usage")
		.arg(
			Arg::with_name("FORMAT")
				.long("format")
				.short("f").help("Output format\n<default>: term table format\nmarkdown: markdown format\nplain: term plain format")
				.takes_value(true)
				.required(false))
		.arg(
			Arg::with_name("SEARCH")
				.long("search")
				.short("s").help("")
				.takes_value(true)
				.required(false))
}

pub fn run<'a, 'b>(matches: &ArgMatches, modules: &Vec<Module>) -> Result<Vec<String>, String> {
	let usage_info = get_usage_info(modules);

	let search = matches.value_of("SEARCH");

	let usage_info = match search {
		Some(search) => {
			let search = &search.to_lowercase();

			let usage_info = usage_info.into_iter().filter_map(|(title, commands)|{
				let commands = commands.into_iter().filter(|item| {
					let mut row = vec![&item.0, &item.1, &item.2, &item.3];
					row.extend(&item.4);
					row.iter().any(|&cell| cell.to_lowercase().contains(search))
				}).collect::<Vec<(String, String, String, String, Vec<String>)>>();
				if commands.len() > 0 {
					Some((title, commands))
				}else{
					None
				}
			}).collect();
			usage_info
		}
		_ => usage_info,
	};

	let format = matches.value_of("FORMAT");

	match format {
		Some("markdown") => markdown_output(usage_info),
		Some("plain") => term_plain_output(usage_info),
		_ => term_table_output(usage_info),
	}
}

fn term_plain_output(usage_info: Vec<(String, Vec<(String, String, String, String, Vec<String>)>)>) -> Result<Vec<String>, String> {
	const WIDTH: usize = 100;

	let mut result = vec![
		"Usage".to_string(),
	];

	let body = usage_info.into_iter().fold(Vec::new(), |mut vec, (_, mut commands)| {
		vec.append(&mut commands);
		vec
	});
	let body = body.into_iter().fold(Vec::new(), |mut vec, items| {
		let tmp = vec![format!("# {}", items.0), items.1, items.2, items.3].into_iter()
			.filter(|x| x.len() > 0).map(|x| add_enter(x, WIDTH, false))
			.chain(items.4.into_iter().map(|x| add_enter(x, WIDTH, true))).collect::<Vec<String>>();
		vec.extend(tmp);
		vec.push("".to_string());
		vec
	});

	result.extend(body);

	Ok(result)
}

fn term_table_output(usage_info: Vec<(String, Vec<(String, String, String, String, Vec<String>)>)>) -> Result<Vec<String>, String> {

	const DESC_WIDTH: usize = 40;
	const EXAMPLE_WIDTH: usize = 60;

	let mut result = vec![
		"Usage".to_string(),
	];

	let body = usage_info.into_iter().fold(Vec::new(), |mut vec, (_, mut commands)| {
		vec.append(&mut commands);
		vec
	});
	let mut table = Table::init(
		body.into_iter().map(|item| {
			let sub_command = item.0;
			let desc = vec![item.1, item.2, item.3].into_iter()
				.filter(|x| x.len() > 0).map(|x| add_enter(x.to_owned(), DESC_WIDTH, false))
				.collect::<Vec<String>>().join("\n");
			let example = item.4.into_iter().map(|x| add_enter(x.to_string(), EXAMPLE_WIDTH, true)).collect::<Vec<String>>().join("\n");
			Row::new(vec![Cell::new(&sub_command), Cell::new(&desc), Cell::new(&example)])
		}
		).collect()
	);
	table.set_format(*format::consts::FORMAT_NO_COLSEP);

	result.push(table.to_string());

	Ok(result)
}

fn markdown_output(usage_info: Vec<(String, Vec<(String, String, String, String, Vec<String>)>)>) -> Result<Vec<String>, String> {

	const DESC_WIDTH: usize = 40;
	const EXAMPLE_WIDTH: usize = 60;

	let mut result = vec![
		"# Usage".to_string(),
		"".to_string(),
	];

	// table of contents
	let outline = usage_info.iter().map(|(title, _)|{
		format!("- [{}](#{})", title, anchor(title))
	}).collect::<Vec<String>>();

	result.push("## Table of Contents".to_string());
	result.extend(outline);
	result.push("".to_string());

	// body
	let blanks_re = regex::Regex::new(" {2,}").expect("qed");

	let cell_process = |cell: String| {
		let cell = escaper::encode_minimal(&cell);
		// blank(more then 1) to &nbsp;
		let cell = blanks_re.replace_all(&cell, |caps: &Captures| {
			caps[0].replace(" ", "&nbsp;")
		});
		// enter to <br>
		let cell = cell.replace("\\\n", "\\\\<br>");
		cell.replace("\n", "<br>")
	};

	let header = vec!["Sub command",  "Desc", "Example"];

	for module in usage_info {
		result.push(format!("## {}", module.0));
		result.push("".to_string());
		let body = module.1;
		let body = body.into_iter().map(|item| {
			let sub_command = item.0;
			let desc = vec![item.1, item.2, item.3].into_iter()
				.filter(|x| x.len() > 0).map(|x| add_enter(x.to_owned(), DESC_WIDTH, false))
				.collect::<Vec<String>>().join("\n");
			let example = item.4.into_iter().map(|x| add_enter(x.to_string(), EXAMPLE_WIDTH, true)).collect::<Vec<String>>().join("\n");
			vec![sub_command, desc, example]
		}).collect::<Vec<Vec<String>>>();

		let body: Vec<LinkedHashMap<String, String>> = body.into_iter().map(|row| {
			row.into_iter().enumerate().map(|(col, cell)| (header[col].to_string(), cell_process(cell))).collect::<LinkedHashMap<String, String>>()
		}).collect();

		let table = mk_table(&body, &None);
		result.push(table);
		result.push("".to_string());
		result.push("".to_string());
	}

	Ok(result)
}

/// Get usage info
/// Sub command, Sub command desc, Case desc, Since, Example
fn get_usage_info(modules: &Vec<Module>) -> Vec<(String, Vec<(String, String, String, String, Vec<String>)>)> {
	let mut result = vec![];
	for module in modules {
		let module_desc = module.desc.clone();
		let mut body = vec![];
		let commands = &module.commands;
		let cases = (module.get_cases)();
		for command in commands {
			let name = command.app.get_name();
			let about = get_about(&command.app);
			let cases = cases.get(name);
			if let Some(cases) = cases {
				for case in cases {
					if case.is_example {
						let sub_command = name.to_owned();
						let sub_command_desc = about.clone();
						let case_desc = case.desc.clone();
						let since = format!("v{}", case.since);
						let example = {
							let input = case.input.clone();
							let output = case.output.clone();
							let input = vec!["$ dtool".to_string(), name.to_string()].into_iter().chain(input).collect::<Vec<String>>().join(" ");
							vec![input].into_iter().chain(output).collect::<Vec<String>>()
						};

						let item = (sub_command, sub_command_desc, case_desc, since, example);
						body.push(item);
					}
				}
			}
		}
		result.push((module_desc, body));
	}
	result
}

fn get_about(app: &App) -> String {
	let mut help = Vec::new();
	let _ = app.write_help(&mut help);

	let about = String::from_utf8(help)
		.map(|x| x.split("\n").skip(1).take(1).collect::<String>());//about is the second line

	about.ok().unwrap_or_default()
}

fn add_enter(data: String, len: usize, add_escape: bool) -> String {
	let mut v = vec![];
	let mut cur = data.as_str();
	while !cur.is_empty() {
		let (chunk, rest) = cur.split_at(min(len, cur.len()));
		v.push(chunk);
		cur = rest;
	}
	let join_char = if add_escape { "\\\n" } else { "\n" };
	v.join(join_char)
}

fn anchor(title: &str) -> String{
	title.chars().filter_map(|x| {
		if x.is_whitespace() ||  x == '-' {
			Some('-')
		} else if x.is_alphanumeric() {
			Some(x.to_lowercase().nth(0).expect("qed"))
		} else{
			None
		}
	}).collect::<String>()
}
