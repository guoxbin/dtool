use clap::{App, ArgMatches};
use std::collections::HashMap;
mod hex;

pub struct Command<'a, 'b> {
	pub app: App<'a, 'b>,
	pub f: fn(&ArgMatches<'a>) -> (),
}

pub struct ModuleManager<'a, 'b>{
	commands : HashMap<String, Command<'a, 'b>>,
}

impl<'a, 'b> ModuleManager<'a, 'b> {

	pub fn new() -> Self {
		let mut mm = Self {
			commands: HashMap::new(),
		};
		mm.register(hex::commands());
		mm
	}

	pub fn apps(&self) -> Vec<App<'a, 'b>> {

		self.commands.iter().map(|(_, command)| command.app.to_owned()).collect()

	}

	pub fn run(&self, name: &str, matches: &ArgMatches<'a>) {

		if let Some(command) = self.commands.get(name){
			(command.f)(matches);
		}

	}

	fn register(&mut self, commands: Vec<Command<'a, 'b>>) {
		for command in commands {
			self.commands.insert(command.app.get_name().to_string(), command);
		}
	}

}
