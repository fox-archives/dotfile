use colored::Colorize;
use std::{
	collections::HashMap,
	env, fs, os,
	path::PathBuf,
	process::{exit, Command, Stdio},
};

use glob::glob;

use crate::{cli::ReconcileCommands, config::Config};

use crate::tui;

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

pub enum DotfileEntryOp {
	Symlink,
}

pub struct DotfileEntry {
	pub op: DotfileEntryOp,
	// Path in ~/.dotfiles/
	pub source: PathBuf,
	// Path in ~/
	pub target: PathBuf,
}

pub fn get_dotfile_list(config: &Config) -> Result<Vec<DotfileEntry>, std::io::Error> {
	let deploy_sh = match env::consts::OS {
		"windows" => PathBuf::from(&config.dotmgr_dir).join("impl/deploy.ps1"),
		_ => PathBuf::from(&config.dotmgr_dir).join("impl/deploy.sh"),
	};

	let mut dotfiles = vec![];

	let output = match env::consts::OS {
		"windows" => Command::new("pwsh.exe").arg(deploy_sh).output()?,
		_ => Command::new(deploy_sh).output()?,
	};

	if !output.status.success() {
		eprintln!("Failed to execute deploy script");
		eprintln!("{}", String::from_utf8_lossy(&output.stderr));
		exit(1);
	}

	for line in String::from_utf8_lossy(&output.stdout).split("\n") {
		if line.is_empty() || line.starts_with("#") {
			continue;
		}

		let parts: Vec<&str> = line.split("|").collect();

		if parts.len() != 3 {
			eprintln!("warning: line does not have two semicolons: {}", line);
			continue;
		}

		dotfiles.push(DotfileEntry {
			op: DotfileEntryOp::Symlink,
			source: PathBuf::from(parts[1]),
			target: PathBuf::from(parts[2]),
		});
	}

	Ok(dotfiles)
}

type ReconcilerFn = fn(source: PathBuf, target: PathBuf);

struct Reconciler {
	pub status: ReconcilerFn,
	pub deploy: ReconcilerFn,
	pub undeploy: ReconcilerFn,
}

// fn prettify_path(path: &PathBuf) -> PathBuf {
// 	let home = dirs::home_dir().unwrap();
// 	let home = home.to_str().unwrap();

// 	if path.starts_with(&home) {
// 		let b: String = String::from(path.to_str().unwrap())
// 			.chars()
// 			.skip(home.len() + 1)
// 			.collect();
// 		PathBuf::from(String::from("~/") + b.as_str())
// 	} else {
// 		PathBuf::from(path)
// 	}
// }

fn parent_mkdirp(path: PathBuf) {
	fs::create_dir_all(path.parent().unwrap()).unwrap();
}

#[cfg(target_os = "windows")]
fn symlink(original: PathBuf, target: PathBuf) {
	if original.is_dir() {
		os::windows::fs::symlink_dir(original, target).unwrap();
	} else if original.is_file() {
		os::windows::fs::symlink_file(original, target).unwrap();
	}
}

#[cfg(not(target_os = "windows"))]
fn symlink(original: PathBuf, target: PathBuf) {
	if target.is_symlink() {
		fs::remove_file(&target).unwrap();
		os::unix::fs::symlink(original, target).unwrap();
	} else {
		panic!("Bad symlink");
	}
}

fn unsymlink(target: &PathBuf) {
	if target.is_symlink() {
		fs::remove_file(target).unwrap();
	} else {
		println!(
			"WARNING: Cannot handle path (not symlink): {}",
			target.to_str().unwrap()
		);
	}
}

fn print_title(target: &PathBuf) {
	let basename = String::from(target.parent().unwrap().to_str().unwrap());
	let filename = String::from(target.file_name().unwrap().to_str().unwrap());

	let output = format!(
		"{}{}{}",
		basename.white(),
		String::from(std::path::MAIN_SEPARATOR).dimmed(),
		filename.blue()
	);
	println!("👉 {}", output.as_str());
}

fn print_fixable(fixable: bool) {
	if fixable {
		println!("  => {} yes", "fixable:".dimmed());
	} else {
		println!("  => {} no", "fixable:".dimmed());
	}
}

pub fn reconcile_dotfiles(dotfiles: Vec<DotfileEntry>, reconciler_command: ReconcileCommands) {
	let run =
		|reconciler_command: &ReconcileCommands, dotfile: &DotfileEntry, reconciler: Reconciler| {
			match reconciler_command {
				ReconcileCommands::Status {} => {
					(reconciler.status)(dotfile.source.clone(), dotfile.target.clone())
				}
				ReconcileCommands::Deploy {} => {
					(reconciler.deploy)(dotfile.source.clone(), dotfile.target.clone())
				}
				ReconcileCommands::Undeploy {} => {
					(reconciler.undeploy)(dotfile.source.clone(), dotfile.target.clone())
				}
			}
		};

	for dotfile in dotfiles {
		if dotfile.source.is_symlink() {
			if dotfile.target.is_symlink() {
			} else if dotfile.target.is_file() {
			} else if dotfile.target.is_dir() {
			} else if !dotfile.target.exists() {
			}
		} else if dotfile.source.is_file() {
			if dotfile.target.is_symlink() {
				run(
					&reconciler_command,
					&dotfile,
					Reconciler {
						status: |_source, target| {
							print_title(&target);
							print_fixable(true);
						},
						deploy: |source, target| {
							parent_mkdirp(target.clone());
							symlink(source, target);
						},
						undeploy: |_, target| {
							unsymlink(&target);
						},
					},
				);
			} else if dotfile.target.is_file() {
			} else if dotfile.target.is_dir() {
			} else if !dotfile.target.exists() {
			}
		} else if dotfile.source.is_dir() {
			if dotfile.target.is_symlink() {
				run(
					&reconciler_command,
					&dotfile,
					Reconciler {
						status: |_source, target| {
							print_title(&target);
							print_fixable(true);
						},
						deploy: |source, target| {
							parent_mkdirp(target.clone());
							symlink(source, target);
						},
						undeploy: |_, target| {
							unsymlink(&target);
						},
					},
				);
			} else if dotfile.target.is_file() {
			} else if dotfile.target.is_dir() {
			} else if !dotfile.target.exists() {
			} else {
				eprintln!(
					"Does not know how to handle file: {}",
					dotfile.target.to_str().unwrap()
				);
			}
		} else if !dotfile.source.exists() {
			if dotfile.target.is_symlink() {
			} else if dotfile.target.is_file() {
			} else if dotfile.target.is_dir() {
			} else if !dotfile.target.exists() {
			}
		}

		println!(
			"source: {}: {}",
			dotfile.source.to_str().unwrap(),
			dotfile.source.is_dir()
		);
		println!(
			"target: {}: {}",
			dotfile.target.to_str().unwrap(),
			dotfile.target.is_symlink()
		);
		println!("");
	}
}
