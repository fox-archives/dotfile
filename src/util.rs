use std::{
	collections::HashMap,
	env, fmt,
	fs::{self, File},
	io::{self, BufRead, BufReader},
	path::PathBuf,
	process::{exit, Command, Stdio},
};

use glob::glob;

use crate::{cli::ReconcileCommands, tui};

pub struct Config {
	pub dotfiles_dir: PathBuf,
	pub dotmgr_src_dir: PathBuf,
	pub dotmgr_dir: PathBuf,
}

impl Default for Config {
	fn default() -> Config {
		let dotfiles_dir = match env::var("DOTMGR_DOTFILES_DIR") {
			Ok(val) => {
				let p = PathBuf::from(val);
				if p.is_absolute() {
					p
				} else {
					PathBuf::from(env::var("HOME").unwrap()).join(".dotfiles")
				}
			}
			Err(_err) => PathBuf::from(env::var("HOME").unwrap()).join(".dotfiles"),
		};

		Config {
			dotfiles_dir: dotfiles_dir.clone(),
			dotmgr_src_dir: dotfiles_dir.clone().join(".dotmgr"),
			dotmgr_dir: dotfiles_dir.join("dotmgr"),
		}
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"dotfiles_dir: {}\ndotmgr_src_dir: {}\ndotmgr_dir: {}",
			self.dotfiles_dir.to_str().unwrap(),
			self.dotmgr_src_dir.to_str().unwrap(),
			self.dotmgr_dir.to_str().unwrap()
		)
	}
}

pub fn get_config() -> Config {
	let mut config = Config::default();

	let config_file = File::open(config.dotfiles_dir.join("dotmgr.conf"));
	if config_file.is_err() {
		return config;
	}

	let reader = BufReader::new(config_file.unwrap());
	for line in reader.lines() {
		let l = line.unwrap().to_string();
		match l.find("=") {
			Some(val) => {
				let key = l.get(..val).unwrap().trim();
				let value = l.get(val + 1..).unwrap().trim();

				if key == "dotmgr_src_dir" {
					if value.starts_with("~/") {
						let v: String = value.chars().skip(2).collect();
						config.dotmgr_src_dir = PathBuf::from(env::var("HOME").unwrap()).join(v);
					} else {
						config.dotmgr_src_dir = PathBuf::from(value);
					}
				} else if key == "dotmgr_dir" {
					if value.starts_with("~/") {
						let v: String = value.chars().skip(2).collect();
						config.dotmgr_dir = PathBuf::from(env::var("HOME").unwrap()).join(v);
					} else {
						config.dotmgr_dir = PathBuf::from(value);
					}
				}
			}
			None => {
				println!("no match");
			}
		}
	}

	return config;
}

pub fn get_script_exec(dir: PathBuf, glob_pattern: Option<String>) -> PathBuf {
	match glob_pattern {
		Some(ref val) => {
			let s = String::from(format!("{}/*{}*", dir.to_str().unwrap(), val.as_str()));

			let mut paths: Vec<PathBuf> = vec![];
			for result in glob(s.as_str()).unwrap() {
				paths.push(result.unwrap())
			}

			if paths.len() > 1 {
				eprintln!("Your string matches more than one file, try again");
				exit(1);
			}

			let mut script = paths.first().unwrap().to_owned();
			if String::from(script.to_str().unwrap()).is_empty() {
				eprintln!("The found script is empty");
				exit(1);
			}

			if script.is_dir() {
				script = script.join("script.sh");
			}

			env::set_current_dir(script.parent().unwrap()).unwrap();

			if !script.exists() {
				eprintln!("Script does not exist: {}", script.to_str().unwrap());
				exit(1);
			}

			PathBuf::from(script)
		}
		None => {
			return tui::choose_script(dir);
		}
	}
}

pub fn get_entrypoint_sh(dotmgr_dir: &str) -> PathBuf {
	return PathBuf::from(dotmgr_dir).join("impl/entrypoint.sh");
}

fn get_environment_sh(dotmgr_dir: &str) -> PathBuf {
	return PathBuf::from(dotmgr_dir).join("impl/environment.sh");
}

pub fn get_environment(
	config: &Config,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
	let mut map = HashMap::new();

	let environment_script = get_environment_sh(config.dotmgr_dir.to_str().unwrap());
	let child = Command::new(environment_script)
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute child");

	let output = child.wait_with_output().expect("failed to wait on child");

	for line in String::from_utf8(output.stdout.clone())?.lines() {
		if line.starts_with("#") {
			continue;
		}

		match line.find("=") {
			Some(val) => {
				let key = line[..val].trim().replace('"', "");
				let value = line[val + 1..].trim().replace('"', "");

				map.insert(String::from(key), String::from(value));
			}
			None => {}
		}
	}

	Ok(map)
}

pub fn get_sources(dotmgr_dir: &str) -> String {
	let mut paths: Vec<String> = Vec::new();

	let dir = PathBuf::from(dotmgr_dir).join("util");

	for result in glob(format!("{}/*.sh", dir.to_str().unwrap()).as_str()).unwrap() {
		let s = String::from(result.unwrap().to_str().unwrap());
		paths.push(s);
	}

	paths.join(":")
}

pub fn get_editor() -> String {
	match env::var("VISUAL") {
		Ok(val) => val,
		Err(..) => match env::var("EDITOR") {
			Ok(val) => val,
			Err(..) => String::from("vi"),
		},
	}
}

pub fn get_pager() -> String {
	if does_command_exist("bat", "--help") {
		String::from("bat")
	} else {
		match env::var("PAGER") {
			Ok(val) => val,
			Err(..) => String::from("less"),
		}
	}
}

pub fn does_command_exist(command_name: &str, help_flag: &str) -> bool {
	let mut command = Command::new(command_name);
	command.arg(help_flag);
	command.stdin(Stdio::null());
	command.stdout(Stdio::null());
	command.stderr(Stdio::null());

	if let Ok(mut child) = command.spawn() {
		child.wait().unwrap();
		true
	} else {
		false
	}
}
