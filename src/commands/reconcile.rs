use core::error::Source;
use std::{
	collections::HashMap,
	env, fs, os,
	path::PathBuf,
	process::{exit, Command, Stdio},
};

use colored::Colorize;
use glob::glob;

use crate::util;
use crate::{cli::ReconcileCommands, config::Config, tui};

pub struct CommandReconcile {
	config: Config,
	dotfile_list: Vec<DotfileEntry>,
}

impl CommandReconcile {
	pub fn new(config: Config) -> Self {
		let dotfile_list = get_dotfile_list(&config).unwrap();

		Self {
			config,
			dotfile_list,
		}
	}

	pub fn status(&self) {
		reconcile_dotfiles(&self.dotfile_list, ReconcileCommands::Status {});
	}

	pub fn deploy(&self) {
		reconcile_dotfiles(&self.dotfile_list, ReconcileCommands::Deploy {});
	}

	pub fn undeploy(&self) {
		reconcile_dotfiles(&self.dotfile_list, ReconcileCommands::Undeploy {});
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

pub fn reconcile_dotfiles(dotfiles: &Vec<DotfileEntry>, reconciler_command: ReconcileCommands) {
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
				run(
					&reconciler_command,
					&dotfile,
					Reconciler {
						status: |_, target| {
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
				)
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
							fs::remove_file(target).unwrap();
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
