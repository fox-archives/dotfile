use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	#[clap(flatten)]
	pub verbose: Verbosity,

	#[command(subcommand)]
	pub command: CliCommands,
}

#[derive(Subcommand)]
pub enum CliCommands {
	/// Operate on a script
	Script {
		/// Choose where to select scripts from
		#[arg(short)]
		category: Option<String>,

		#[command(subcommand)]
		command: ScriptCommands,
	},

	/// Reconcile the dotfiles
	Reconcile {
		#[command(subcommand)]
		command: ReconcileCommands,
	},

	/// Generate executable files form all scripts
	Generate {},

	/// Update dotmgr to the latest release
	Update {},

	/// Run an internal command
	Internal {
		#[command(subcommand)]
		command: InternalCommands,
	},
}

#[derive(Subcommand)]
pub enum ScriptCommands {
	/// List scripts
	List {},

	/// View a script
	View { glob: Option<String> },

	/// Edit a script
	Edit { glob: Option<String> },

	/// Run a script
	Run {
		/// run the script with elevated priviledges
		#[arg(long)]
		sudo: bool,

		glob: Option<String>,
	},
}

#[derive(Subcommand)]
pub enum ReconcileCommands {
	/// View status of dotfiles
	Status {},

	/// Deploy dotfiles
	Deploy {},

	// Undeploy dotfiles
	Undeploy {},
}

#[derive(Subcommand)]
pub enum InternalCommands {
	StartWatcher {},

	FindMan { command_line: String },
}
