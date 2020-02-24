use crate::modules::ModuleManager;
use clap::App;

pub fn build_app<'a, 'b>() -> (App<'a, 'b>, ModuleManager<'a, 'b>) {
	let mut app = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"));

	let module_manager = ModuleManager::new();
	let subcommands = module_manager.apps();

	for subcommand in subcommands {
		app = app.subcommand(subcommand);
	}

	(app, module_manager)
}
