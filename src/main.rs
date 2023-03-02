use clap::Parser;

mod cli;
mod commands;
mod config;
mod tui;
mod util;

use crate::cli::{Cli, CliCommands, InternalCommands, ReconcileCommands, ScriptCommands};
use crate::commands::{CommandInternal, CommandReconcile, CommandScript};
use crate::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = Cli::parse();
	let config = Config::default();

	env_logger::Builder::new()
		.filter_level(cli.verbose.log_level_filter())
		.init();

	match &cli.command {
		CliCommands::Script { command, category } => {
			let command_script = CommandScript::new(config, category.clone());

			match &command {
				ScriptCommands::List {} => {
					command_script.list();
				}
				ScriptCommands::View { glob } => {
					command_script.view(glob.clone());
				}
				ScriptCommands::Edit { glob } => {
					command_script.edit(glob.clone());
				}
				ScriptCommands::Run { glob, sudo } => {
					command_script.run(glob.clone(), sudo.clone());
				}
			}
		}
		CliCommands::Reconcile { command } => {
			let command_reconcile = CommandReconcile::new(config);

			match &command {
				ReconcileCommands::Status {} => {
					command_reconcile.status();
				}
				ReconcileCommands::Deploy {} => {
					command_reconcile.deploy();
				}
				ReconcileCommands::Undeploy {} => {
					command_reconcile.undeploy();
				}
			}
		}
		CliCommands::Internal { command } => {
			let command_internal = CommandInternal::new(config);

			match command {
				InternalCommands::StartWatcher {} => {
					command_internal.start_watcher();
				}
				InternalCommands::FindMan { command_line } => {
					command_internal.find_man();
				}
				InternalCommands::Generate {} => {
					command_internal.generate();
				}
			}
		}
	}

	Ok(())
}
