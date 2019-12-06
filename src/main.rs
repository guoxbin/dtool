use clap::{App};

mod modules;

fn main() {

    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));

    let module_manager = modules::ModuleManager::new();
    let subcommands = module_manager.apps();

    for subcommand in subcommands {
        app = app.subcommand(subcommand);
    }

    let matches = app.get_matches();

    let (name, matches) = matches.subcommand();

    if let Some(matches) = matches {
        module_manager.run(name, matches);
    }

}
