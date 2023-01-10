use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	pub command: CliCommands,
}

#[derive(Subcommand)]
pub enum CliCommands {
	/// Perform an action on a script
	Script {
		/// view the script with a pager
		#[arg(short, long)]
		dir: Option<String>,

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

	/// Updates dotmgr to the latest release
	Update {},
}

#[derive(Subcommand)]
pub enum ScriptCommands {
	/// List scripts
	List {},

	/// View a script
	View { glob: Option<String> },

	/// Edit a script
	Edit { glob: String },

	/// Run a script
	Run {
		/// run the script with elevated priviledges
		#[arg(long)]
		sudo: bool,

		glob: String,
	},
}

#[derive(Subcommand)]
pub enum ReconcileCommands {
	/// Synchronize dotfiles
	Sync {},

	UnSync {},
}
