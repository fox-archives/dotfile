use clap::Parser;
use colored::Colorize;
use notify::{PollWatcher, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind};
use std::{
	path::{Path, PathBuf},
	process::{exit, Command},
	time::Duration,
};

mod cli;
use crate::cli::{Cli, CliCommands, InternalCommands, ReconcileCommands, ScriptCommands};

mod config;
use crate::config::Config;

mod tui;

mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = Cli::parse();
	let config = Config::default();

	env_logger::Builder::new()
		.filter_level(cli.verbose.log_level_filter())
		.init();

	match &cli.command {
		CliCommands::Script { command, category } => {
			let category_path = PathBuf::from(&config.dotmgr_dir).join(match category {
				Some(val) => format!("scripts-{val}"),
				None => String::from("scripts"),
			});

			match &command {
				ScriptCommands::List {} => {
					std::process::Command::new("ls")
						.args(["-x", category_path.to_str().unwrap()])
						.spawn()
						.unwrap();
				}
				ScriptCommands::View { glob } => {
					let script = util::get_script_exec(category_path, glob.clone());
					let pager = util::get_pager();

					log::info!("pager: {}", pager);

					Command::new(pager)
						.arg(script)
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
				ScriptCommands::Edit { glob } => {
					let script = util::get_script_exec(category_path, glob.clone());
					let editor = util::get_editor();

					log::info!("editor: {}", editor);

					Command::new(editor)
						.args([script])
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
				ScriptCommands::Run { glob, sudo } => {
					if *sudo {
						println!("{}", "Not implemented".italic());
						exit(1);
					}

					let env = util::get_environment(&config)?;
					let entrypoint =
						util::get_entrypoint_sh(config.dotmgr_dir.to_str().unwrap().clone());
					let script = util::get_script_exec(category_path, glob.clone());
					let sources = util::get_sources(config.dotmgr_dir.to_str().unwrap().clone());

					for (key, value) in &env {
						log::info!("env: {key}: {value}")
					}
					log::info!("entrypoint: {}", entrypoint.to_str().unwrap());
					log::info!("script: {}", script.to_str().unwrap());
					for source in sources.split(":") {
						log::info!("source: {}", source);
					}

					Command::new(entrypoint)
						.arg(script)
						.arg(sources)
						.envs(env)
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
			}
		}
		CliCommands::Reconcile { command } => {
			let dotfile_list = util::get_dotfile_list(&config).unwrap();

			match &command {
				ReconcileCommands::Status {} => {
					util::reconcile_dotfiles(dotfile_list, ReconcileCommands::Status {});
				}
				ReconcileCommands::Deploy {} => {
					util::reconcile_dotfiles(dotfile_list, ReconcileCommands::Deploy {});
				}
				ReconcileCommands::Undeploy {} => {
					util::reconcile_dotfiles(dotfile_list, ReconcileCommands::Undeploy {});
				}
			}
		}
		CliCommands::Generate {} => {
			println!("{}", "Not implemented".italic());
			exit(1);
		}
		CliCommands::Internal { command } => match command {
			InternalCommands::StartWatcher {} => {
				println!("Starting watcher");

				let (tx, rx) = std::sync::mpsc::channel();
				// This example is a little bit misleading as you can just create one Config and use it for all watchers.
				// That way the pollwatcher specific stuff is still configured, if it should be used.
				let mut watcher: Box<dyn Watcher> =
					if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
						// custom config for PollWatcher kind
						// you
						let config = notify::Config::default().with_poll_interval(Duration::from_secs(1));
						Box::new(PollWatcher::new(tx, config).unwrap())
					} else {
						// use default config for everything else
						Box::new(RecommendedWatcher::new(tx, notify::Config::default()).unwrap())
					};

				// watch some stuff
				watcher
					.watch(Path::new("."), RecursiveMode::Recursive)
					.unwrap();

				// just print all events, this blocks forever
				for e in rx {
					println!("{:?}", e);
				}
			}
			InternalCommands::FindMan { command_line } => {
				let str = util::find_man(command_line.clone());
				println!("{}", str.as_str());
			}
		},
	}

	Ok(())
}
