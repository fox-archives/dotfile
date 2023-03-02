use crate::config::Config;
use colored::Colorize;
use std::path::PathBuf;
use std::process::{exit, Command};

use crate::util;

pub struct CommandScript {
	config: Config,
	category_path: PathBuf,
}

impl CommandScript {
	pub fn new(config: Config, category: Option<String>) -> Self {
		let category_path = PathBuf::from(&config.dotmgr_dir).join(match category {
			Some(val) => format!("scripts-{val}"),
			None => String::from("scripts"),
		});

		Self {
			config,
			category_path,
		}
	}

	pub fn list(&self) {
		std::process::Command::new("ls")
			.args(["-x", self.category_path.to_str().unwrap()])
			.spawn()
			.unwrap();
	}

	pub fn view(&self, glob_pattern: Option<String>) {
		let script = util::get_script_exec(self.category_path.clone(), glob_pattern);
		let pager = util::get_pager();

		log::info!("pager: {}", pager);

		Command::new(pager)
			.arg(script)
			.spawn()
			.unwrap()
			.wait()
			.unwrap();
	}

	pub fn edit(&self, glob_pattern: Option<String>) {
		let script = util::get_script_exec(self.category_path.clone(), glob_pattern);
		let editor = util::get_editor();

		log::info!("editor: {}", editor);

		Command::new(editor)
			.args([script])
			.spawn()
			.unwrap()
			.wait()
			.unwrap();
	}

	pub fn run(&self, glob_pattern: Option<String>, sudo: bool) {
		if sudo {
			println!("{}", "Not implemented".italic());
			exit(1);
		}

		let env = util::get_environment(&self.config).unwrap();
		let entrypoint = util::get_entrypoint_sh(self.config.dotmgr_dir.to_str().unwrap().clone());
		let script = util::get_script_exec(self.category_path.clone(), glob_pattern);
		let sources = util::get_sources(&self.config.dotmgr_dir.to_str().unwrap().clone());

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
