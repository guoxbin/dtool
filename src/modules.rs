use clap::{App, ArgMatches};
use std::iter;
use linked_hash_map::LinkedHashMap;
mod hex;
mod base;
mod time;
mod number_system;
mod base58;
mod base64;
mod url;
mod number_codec;
mod hash;
mod unicode;
mod html;
mod re;
mod usage;
mod pbkdf2;
mod case;

pub struct Command<'a, 'b> {
	pub app: App<'a, 'b>,
	pub f: fn(&ArgMatches<'a>) -> Result<Vec<String>, String>,
	pub cases: Vec<Case>,
}

pub struct Case {
	pub desc: String,
	pub input: Vec<String>,
	pub output: Vec<String>,
	pub is_example: bool,
	pub is_test: bool,
	pub since: String,
}

pub struct ModuleManager<'a, 'b>{
	commands : LinkedHashMap<String, Command<'a, 'b>>,
}

impl<'a, 'b> ModuleManager<'a, 'b> {

	pub fn new() -> Self {
		let mut mm = Self {
			commands: LinkedHashMap::new(),
		};
		mm.register(hex::commands());
		mm.register(time::commands());
		mm.register(number_system::commands());
		mm.register(base58::commands());
		mm.register(base64::commands());
		mm.register(url::commands());
		mm.register(number_codec::commands());
		mm.register(hash::commands());
		mm.register(unicode::commands());
		mm.register(html::commands());
		mm.register(re::commands());
		mm.register(pbkdf2::commands());
		mm.register(case::commands());
		mm
	}

	pub fn apps(&self) -> Vec<App<'a, 'b>> {

		self.commands.iter().map(|(_, command)| command.app.to_owned()).chain( iter::once(usage::usage_app()) ).collect()

	}

	pub fn run(&self, name: &str, matches: &ArgMatches<'a>) {

		if let Some(command) = self.commands.get(name){
			match (command.f)(matches){
				Ok(result) => result.iter().for_each(|x|println!("{}", x)),
				Err(e) => eprintln!("{}", e),
			}
		}

		if name == "usage" {
			match usage::usage(matches, &self.commands){
				Ok(result) => result.iter().for_each(|x|println!("{}", x)),
				Err(e) => eprintln!("{}", e),
			}
		}

	}

	fn register(&mut self, commands: Vec<Command<'a, 'b>>) {
		for command in commands {
			self.commands.insert(command.app.get_name().to_string(), command);
		}
	}

}
