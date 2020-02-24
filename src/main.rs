mod app;
mod modules;

fn main() {
	let (app, module_manager) = app::build_app();

	let mut app_clone = app.clone();

	let matches = app.get_matches();

	let (name, matches) = matches.subcommand();

	if let Some(matches) = matches {
		module_manager.run(name, matches);
	} else {
		app_clone.print_help().unwrap_or(());
		println!();
	}
}
