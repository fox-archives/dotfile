use std::{
	env,
	fs::File,
	io::{self, BufRead, BufReader},
	path::PathBuf,
	process::{exit, Command},
};

use clap::{Parser, Subcommand};
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use glob::glob;
use std::sync::Mutex;
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Constraint, Direction, Layout},
	style::{Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, Borders, Paragraph},
	Frame, Terminal,
};

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
	View { file: String },

	/// Edit a script
	Edit { file: String },

	/// Run a script
	Run {
		/// run the script with elevated priviledges
		#[arg(long)]
		sudo: bool,

		file: String,
	},
}

#[derive(Subcommand)]
pub enum ReconcileCommands {
	/// Synchronize dotfiles
	Sync {},

	UnSync {},
}
