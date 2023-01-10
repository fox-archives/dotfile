use std::{
	env, fmt,
	fs::File,
	io::{BufRead, BufReader},
	os,
	path::PathBuf,
	process::exit,
};

use glob::glob;

use crate::tui;

pub enum When {
	Before,
	After,
}

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

pub fn glob_script(dir: &str, substr: &str) -> PathBuf {
	let mut s = String::from("");
	s.push_str(dir);
	s.push_str("/*");
	s.push_str(substr);
	s.push_str("*");
	println!("{}", dir);
	let mut paths: Vec<PathBuf> = vec![];
	for result in glob(s.as_str()).unwrap() {
		paths.push(result.unwrap())
	}

	if paths.len() > 1 {
		eprintln!("Your string matches more than one file, try again");
		exit(1);
	}

	paths.first().unwrap().clone()
}

pub fn run_hook(config: &Config, when: When, subcommand_name: &str) {}

pub fn get_editor() -> String {
	match env::var("VISUAL") {
		Ok(val) => val,
		Err(_err) => match env::var("EDITOR") {
			Ok(val) => val,
			Err(_err) => String::from("vi"),
		},
	}
}

pub fn get_sources(dotmgr_dir: &str) -> String {
	let mut paths = String::from("");

	let dir = PathBuf::from(dotmgr_dir).join("util");

	let mut s = String::from(dir.to_str().unwrap());
	s.push_str("/*.");
	s.push_str("sh");

	for result in glob(s.as_str()).unwrap() {
		paths = String::from(paths + String::from(":").as_str()) + result.unwrap().to_str().unwrap();
	}

	paths
}

pub fn get_entrypoint(dotmgr_dir: &str) -> PathBuf {
	return PathBuf::from(dotmgr_dir).join("impl/entrypoint.sh");
}

pub fn get_script_from_glob(dir: PathBuf, glob_pattern: &Option<String>) -> PathBuf {
	match glob_pattern {
		Some(val) => glob_script(dir.to_str().unwrap(), val.as_str()),
		None => tui::show_menu(dir),
	}
}
