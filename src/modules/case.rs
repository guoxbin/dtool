use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::{Command, base, Module};
use heck::{TitleCase, SnakeCase, CamelCase, MixedCase, ShoutySnakeCase, KebabCase};

pub fn module<'a, 'b>() -> Module<'a, 'b> {
	Module {
		desc: "Case conversion (upper, lower, title, camel, pascal, snake, shouty snake, kebab)".to_string(),
		commands: commands(),
		get_cases: cases::cases,
	}
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("case").about("Case conversion")
				.arg(
					Arg::with_name("TYPE")
						.long("type")
						.short("t")
						.help("Case type\nupper: GOOD TOOL\nlower: good tool\ntitle: Good Tool\n\
					camel: goodTool\npascal: GoodTool\nsnake: good_tool\nshouty_snake: GOOD_TOOL\n\
					kebab: good-tool")
						.takes_value(true)
						.required(true))
				.arg(
					Arg::with_name("INPUT")
						.required(false)
						.index(1)),
			f: case,
		},
	]
}

fn case(matches: &ArgMatches) -> Result<Vec<String>, String> {
	let input = base::input_string(matches)?;

	let t = matches.value_of("TYPE").ok_or("Invalid type")?;

	let result = match t {
		"upper" => input.to_uppercase(),
		"lower" => input.to_lowercase(),
		"title" => input.to_title_case(),
		"camel" => input.to_mixed_case(),
		"pascal" => input.to_camel_case(), //heck camel casing is pascal casing
		"snake" => input.to_snake_case(),
		"shouty_snake" => input.to_shouty_snake_case(),
		"kebab" => input.to_kebab_case(),
		_ => return Err("Invalid type".to_string()),
	};

	Ok(vec![result])
}

mod cases {
	use crate::modules::Case;
	use linked_hash_map::LinkedHashMap;

	pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
		vec![
			("case",
			 vec![
				 Case {
					 desc: "Upper case".to_string(),
					 input: vec!["-t", "upper", "'good tool'"].into_iter().map(Into::into).collect(),
					 output: vec!["GOOD TOOL"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Lower case".to_string(),
					 input: vec!["-t", "lower", "'GOOD TOOL'"].into_iter().map(Into::into).collect(),
					 output: vec!["good tool"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Title case".to_string(),
					 input: vec!["-t", "title", "'GOOD TOOL'"].into_iter().map(Into::into).collect(),
					 output: vec!["Good Tool"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Camel case".to_string(),
					 input: vec!["-t", "camel", "'GOOD TOOL'"].into_iter().map(Into::into).collect(),
					 output: vec!["goodTool"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Pascal case".to_string(),
					 input: vec!["-t", "pascal", "'GOOD TOOL'"].into_iter().map(Into::into).collect(),
					 output: vec!["GoodTool"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Snake case".to_string(),
					 input: vec!["-t", "snake", "GoodTool"].into_iter().map(Into::into).collect(),
					 output: vec!["good_tool"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Shouty snake case".to_string(),
					 input: vec!["-t", "shouty_snake", "GoodTool"].into_iter().map(Into::into).collect(),
					 output: vec!["GOOD_TOOL"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
				 Case {
					 desc: "Kebab case".to_string(),
					 input: vec!["-t", "kebab", "GoodTool"].into_iter().map(Into::into).collect(),
					 output: vec!["good-tool"].into_iter().map(Into::into).collect(),
					 is_example: true,
					 is_test: true,
					 since: "0.5.0".to_string(),
				 },
			 ]),
		].into_iter().collect()
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
