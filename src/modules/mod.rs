use clap::{App, ArgMatches};
use std::collections::HashMap;
mod hex;
mod time;
mod number;

pub struct Command<'a, 'b> {
	pub app: App<'a, 'b>,
	pub f: fn(&ArgMatches<'a>) -> Result<Vec<String>, String>,
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
		mm.register(time::commands());
		mm.register(number::commands());
		mm
	}

	pub fn apps(&self) -> Vec<App<'a, 'b>> {

		self.commands.iter().map(|(_, command)| command.app.to_owned()).collect()

	}

	pub fn run(&self, name: &str, matches: &ArgMatches<'a>) {

		if let Some(command) = self.commands.get(name){
			match (command.f)(matches){
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
