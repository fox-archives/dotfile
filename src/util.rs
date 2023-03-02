use std::{
	collections::HashMap,
	env, fs, os,
	path::PathBuf,
	process::{exit, Command, Stdio},
};

use colored::Colorize;
use glob::glob;

use crate::{cli::ReconcileCommands, config::Config, tui};

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

pub fn find_man(command_line: String) -> String {
	let mut line = String::from(command_line.trim());
	if line.starts_with("sudo") {
		line = line.chars().skip("sudo".len()).collect::<String>();
	}
	line = line.trim_start().to_string();

	let mut chars = line.chars();

	// Remove backslashes and quotes
	match chars.next() {
		Some('\\') => {
			line = chars.collect();
		}
		Some('\'') => {
			if let Some(i) = chars.clone().position(|s| s == '\'') {
				if chars.clone().skip(i + 1).next().unwrap_or(' ') == ' ' {
					let part1 = chars.clone().take(i).collect::<String>();
					let part2 = chars.clone().skip(i + 1).collect::<String>();
					line = part1 + &part2;
				}
			}
		}
		Some('\"') => {
			if let Some(i) = chars.clone().position(|c| c == '\"') {
				if chars.clone().skip(i + 1).next().unwrap_or(' ') == ' ' {
					let part1 = chars.clone().take(i).collect::<String>();
					let part2 = chars.clone().skip(i + 1).collect::<String>();
					line = part1 + &part2;
				}
			}
		}
		Some(_) => {}
		None => {}
	}

	// Remove the first argument flag and everything after
	// let mut line_arr: Vec<String> = line.split(" ").to_owned();
	// if let Some(i) = line_arr.clone().position(|s| s.starts_with("-")) {
	// 	line_arr = line_arr.take(i).collect::<std::str::Split<&str>>();
	// }

	line = line.replace(" ", "-");

	return line;
}

#[cfg(test)]
mod tests {
	use super::find_man;

	#[test]
	fn it_works() {
		assert_eq!(find_man(String::from("\\git")), "git");

		assert_eq!(find_man(String::from("git")), "git");
		assert_eq!(find_man(String::from("'git")), "'git");
		assert_eq!(find_man(String::from("'git'")), "git");
		assert_eq!(find_man(String::from("'git' status")), "git-status");

		assert_eq!(find_man(String::from("git")), "git");
		assert_eq!(find_man(String::from("\"git")), "\"git");
		assert_eq!(find_man(String::from("\"git\"")), "git");
		assert_eq!(find_man(String::from("\"git\" status")), "git-status");

		assert_eq!(find_man(String::from("sudo git status")), "git-status");
		assert_eq!(find_man(String::from("sudo 'git' status")), "git-status");
		assert_eq!(find_man(String::from("  sudo 'git' status")), "git-status");

		assert_eq!(find_man(String::from("kubectl status")), "kubectl-status");
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
