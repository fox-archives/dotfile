use std::{
	fs,
	path::PathBuf,
	process::{exit, Command},
};

use crate::cli::ReconcileCommands;

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

pub fn get_deploy_sh(dotmgr_dir: &str) -> PathBuf {
	return PathBuf::from(dotmgr_dir).join("impl/deploy.sh");
}

pub fn get_dotfile_list(deploy_sh: PathBuf) -> Result<Vec<DotfileEntry>, std::io::Error> {
	let mut dotfiles = vec![];

	let output = Command::new(deploy_sh).output()?;
	if !output.status.success() {
		eprintln!("Failed to execute deploy.sh");
		eprintln!("{}", String::from_utf8_lossy(&output.stderr));
		exit(1);
	}

	for line in String::from_utf8_lossy(&output.stdout).split("\n") {
		if line.is_empty() || line.starts_with("#") {
			continue;
		}

		let parts: Vec<&str> = line.split(":").collect();

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

fn print_files(source: PathBuf, target: PathBuf) {
	println!("  source: {}", source.to_str().unwrap());
	println!("  target: {}", target.to_str().unwrap());
}

fn symlink_resolved_properly(symlink: PathBuf, proper_value: String) {
	let c = symlink.read_link().unwrap();
}

pub fn reconcile_dotfiles(dotfiles: Vec<DotfileEntry>, reconciler_command: ReconcileCommands) {
	let reconcilers = Reconcilers {
		symlink_to_symlink: Reconciler {
			status: |source, target| {
				show_reconcile_failure("ERROR_SYMLINK_SYMLINK");
				println!("ERR_SYM_SYM");
				print_files(source, target);
			},
			deploy: |source, target| {},
			undeploy: |_, target| {
				fs::remove_file(target).unwrap();
			},
		},
		symlink_to_file: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		symlink_to_dir: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		symlink_to_null: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		file_to_symlink: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |_, target| {
				fs::remove_file(target).unwrap();
			},
		},
		file_to_file: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		file_to_dir: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		file_to_null: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		dir_to_symlink: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |_, target| {
				fs::remove_file(target).unwrap();
			},
		},
		dir_to_file: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		dir_to_dir: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		dir_to_null: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		null_to_symlink: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |_, target| {
				fs::remove_file(target).unwrap();
			},
		},
		null_to_file: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		null_to_dir: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
		},
		null_to_null: Reconciler {
			status: |source, target| {},
			deploy: |source, target| {},
			undeploy: |source, target| {},
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
