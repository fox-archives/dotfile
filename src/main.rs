#![feature(exit_status_error)]

use clap::Parser;
use cli::{Cli, CliCommands};
use std::{
	collections::HashMap,
	io::{BufRead, BufReader},
	process::{Command, Stdio},
};

mod cli;
use crate::cli::ScriptCommands;

mod tui;

mod util;
use util::{get_entrypoint, When};

fn main() {
	let cli = Cli::parse();
	let config = util::get_config();

	match &cli.command {
		CliCommands::Script { command, dir } => {
			let scripts_dirname = match dir {
				Some(val) => String::from("scripts-") + val,
				None => String::from("scripts/"),
			};
			let scripts_path = config.dotmgr_dir.clone().join(scripts_dirname);

			match &command {
				ScriptCommands::List {} => {
					std::process::Command::new("ls")
						.args(["-x", scripts_path.to_str().unwrap()])
						.spawn()
						.unwrap();
				}
				ScriptCommands::View { glob } => {
					let script = util::get_script_from_glob(scripts_path, glob);

					Command::new("bat")
						.arg(script)
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
				ScriptCommands::Edit { glob } => {
					let script = util::glob_script(scripts_path.to_str().unwrap(), glob);

					Command::new(util::get_editor())
						.args([script])
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
				ScriptCommands::Run { glob, sudo } => {
					let get_environment = || -> HashMap<String, String> {
						let s = config.dotmgr_dir.to_str().unwrap();
						let environment_script = String::from(s) + "/impl/environment.sh";

						let mut child = Command::new(environment_script)
							.stdout(Stdio::piped())
							.spawn()
							.unwrap();

						let mut map = HashMap::new();

						if let Some(stdout) = &mut child.stdout {
							let lines = BufReader::new(stdout).lines();
							for maybe_line in lines {
								let line = maybe_line.unwrap();

								match line.find("=") {
									Some(val) => {
										let key = &line.as_str()[..val];
										let value = &line.as_str()[val + 1..];

										map.insert(String::from(key), String::from(value));
									}
									None => {}
								}
							}
						} else {
							panic!("Something went wrong");
						}

						map
					};

					let entrypoint = get_entrypoint(config.dotmgr_dir.to_str().unwrap().clone());
					let script = util::glob_script(scripts_path.to_str().unwrap(), glob);
					let sources = util::get_sources(config.dotmgr_dir.to_str().unwrap().clone());

					Command::new(entrypoint)
						.arg(script)
						.arg(sources)
						.envs(get_environment())
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
			}
		}
		CliCommands::Reconcile { .. } => {
			println!("Not implemented");
		}
		CliCommands::Generate {} => {
			println!("Not implemented");
		}
		CliCommands::Update {} => {
			util::run_hook(&config, When::Before, "update");

			let dir = config.dotmgr_src_dir.to_str().unwrap();
			Command::new("git")
				.args(["-C", dir, "status", "--short"])
				.spawn()
				.unwrap()
				.wait()
				.unwrap();
			Command::new("git")
				.args(["-C", dir, "pull"])
				.spawn()
				.unwrap()
				.wait()
				.unwrap();
			Command::new("git")
				.args(["-C", dir, "status", "--short"])
				.spawn()
				.unwrap()
				.wait()
				.unwrap();

			util::run_hook(&config, When::After, "update");
		}
	}
}
