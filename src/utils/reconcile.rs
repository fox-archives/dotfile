use core::str;
use std::{
	env, fs, os,
	path::PathBuf,
	process::{exit, Command},
};

use colored::Colorize;
use crossterm::style::Stylize;

use crate::cli::ReconcileCommands;
use crate::util::{self, Config};

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
		_ => Command::new(deploy_sh).output()?
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

pub struct Reconciler {
	pub status: ReconcilerFn,
	pub deploy: ReconcilerFn,
	pub undeploy: ReconcilerFn,
}

pub struct Reconcilers {
	pub symlink_to_symlink: Reconciler,
	pub symlink_to_file: Reconciler,
	pub symlink_to_dir: Reconciler,
	pub symlink_to_null: Reconciler,
	pub file_to_symlink: Reconciler,
	pub file_to_file: Reconciler,
	pub file_to_dir: Reconciler,
	pub file_to_null: Reconciler,
	pub dir_to_symlink: Reconciler,
	pub dir_to_file: Reconciler,
	pub dir_to_dir: Reconciler,
	pub dir_to_null: Reconciler,
	pub null_to_symlink: Reconciler,
	pub null_to_file: Reconciler,
	pub null_to_dir: Reconciler,
	pub null_to_null: Reconciler,
}

fn prettify_path(path: &PathBuf) -> PathBuf {
	let home = dirs::home_dir().unwrap();
	let home = home.to_str().unwrap();

	if path.starts_with(&home) {
		let b: String = String::from(path.to_str().unwrap())
			.chars()
			.skip(home.len() + 1)
			.collect();
		PathBuf::from(String::from("~/") + b.as_str())
	} else {
		PathBuf::from(path)
	}
}

fn symlink_resolved_properly(source: &PathBuf, target: &PathBuf) -> bool {
	if target.is_symlink() {
		let expected_value = target.read_link().unwrap();
		if expected_value == source.clone() {
			true
		} else {
			false
		}
	} else {
		false
	}
}

fn print_path(source: &PathBuf, target: &PathBuf) {
	// let p = prettify_path(&target);
	// let s = String::from(p.to_str().unwrap());
	// let s2 =  String::from(target.to_str().unwrap());
	let target_str = String::from(target.to_str().unwrap());
	println!("{}", target_str.dimmed());
	// println!("{} :: {}", s2, target.exists());

	let status: String;
	if target.exists() {
		if symlink_resolved_properly(&source, &target) {
			status = String::from("GOOD");
		} else {
			status = String::from("TARGET CREATED, WRONG VALUE");
		}
	} else {
		status = String::from("TARGET NOT CREATED");
	}
	println!("  => {}", status.as_str());
}

pub fn handle_unsymlink(target: &PathBuf) {
	if target.is_symlink() {
		fs::remove_file(target).unwrap();
	} else {
		println!(
			"WARNING: Cannot handle path (not symlink): {}",
			target.to_str().unwrap()
		);
	}
}

pub fn reconcile_dotfiles(dotfiles: Vec<DotfileEntry>, reconciler_command: ReconcileCommands) {
	let reconcilers = Reconcilers {
		symlink_to_symlink: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		symlink_to_file: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		symlink_to_dir: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		symlink_to_null: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {
				fs::create_dir_all(target.parent().unwrap()).unwrap();
				util::symlink(source, target);
			},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		file_to_symlink: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		file_to_file: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		file_to_dir: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		file_to_null: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {
				fs::create_dir_all(target.parent().unwrap()).unwrap();
				util::symlink(source, target);
			},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		dir_to_symlink: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		dir_to_file: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		dir_to_dir: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		dir_to_null: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {
				fs::create_dir_all(target.parent().unwrap()).unwrap();
				util::symlink(source, target);
			},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		null_to_symlink: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		null_to_file: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		null_to_dir: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
		null_to_null: Reconciler {
			status: |source, target| {
				print_path(&source, &target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				handle_unsymlink(&target);
			},
		},
	};

	let get_fn = |reconciler: &Reconciler, reconciler_command: &ReconcileCommands| -> ReconcilerFn {
		match reconciler_command {
			ReconcileCommands::Status {} => reconciler.status,
			ReconcileCommands::Deploy {} => reconciler.deploy,
			ReconcileCommands::Undeploy {} => reconciler.undeploy,
		}
	};

	for dotfile in dotfiles {
		fs::create_dir_all(dotfile.target.parent().unwrap()).unwrap();

		if dotfile.source.is_symlink() {
			if dotfile.target.is_symlink() {
				// symlink to symlink
				let f = get_fn(&reconcilers.symlink_to_symlink, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_file() {
				// symlink to file
				let f = get_fn(&reconcilers.symlink_to_file, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_dir() {
				// symlink to dir
				let f = get_fn(&reconcilers.symlink_to_dir, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if !dotfile.target.exists() {
				// symlink to null
				let f = get_fn(&reconcilers.symlink_to_null, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else {
				eprintln!(
					"Does not know how to handle file: {}",
					dotfile.target.to_str().unwrap()
				);
			}
		} else if dotfile.source.is_file() {
			if dotfile.target.is_symlink() {
				// file to symlink
				let f = get_fn(&reconcilers.file_to_symlink, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_file() {
				// file to file
				let f = get_fn(&reconcilers.file_to_file, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_dir() {
				// file to dir
				let f = get_fn(&reconcilers.file_to_dir, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if !dotfile.target.exists() {
				// file to null
				let f = get_fn(&reconcilers.file_to_null, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else {
				eprintln!(
					"Does not know how to handle file: {}",
					dotfile.target.to_str().unwrap()
				);
			}
		} else if dotfile.source.is_dir() {
			if dotfile.target.is_symlink() {
				// dir to symlink
				let f = get_fn(&reconcilers.dir_to_symlink, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_file() {
				// dir to file
				let f = get_fn(&reconcilers.dir_to_file, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_dir() {
				// dir to dir
				let f = get_fn(&reconcilers.dir_to_dir, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if !dotfile.target.exists() {
				// dir to null
				let f = get_fn(&reconcilers.dir_to_null, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else {
				eprintln!(
					"Does not know how to handle file: {}",
					dotfile.target.to_str().unwrap()
				);
			}
		} else if !dotfile.source.exists() {
			if dotfile.target.is_symlink() {
				// null to symlink
				let f = get_fn(&reconcilers.null_to_symlink, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_file() {
				// null to file
				let f = get_fn(&reconcilers.null_to_file, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if dotfile.target.is_dir() {
				// null to dir
				let f = get_fn(&reconcilers.null_to_dir, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else if !dotfile.target.exists() {
				// null to null
				let f = get_fn(&reconcilers.null_to_null, &reconciler_command);
				f(dotfile.source, dotfile.target);
			} else {
				eprintln!(
					"Does not know how to handle file: {}",
					dotfile.target.to_str().unwrap()
				);
			}
		} else {
			eprintln!(
				"Does not know how to handle file: {}",
				dotfile.source.to_str().unwrap()
			)
		}
	}
}
