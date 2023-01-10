#![feature(exit_status_error)]

use clap::Parser;
use cli::{Cli, CliCommands};
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use glob::glob;
use std::{
	collections::HashMap,
	io::{self, BufRead, BufReader},
	path::{Path, PathBuf},
	process::{self, exit, Command, ExitStatus, Stdio},
};
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Constraint, Direction, Layout},
	style::{Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, Borders, Paragraph},
	Frame, Terminal,
};

mod cli;
use crate::cli::ScriptCommands;

mod util;
use util::{get_entrypoint, When};

struct App {
	file_num: u32,
	desc_num: u32,
}

impl App {
	fn new() -> App {
		App {
			file_num: 1,
			desc_num: 2,
		}
	}
}

fn main() {
	let cli = Cli::parse();
	let config = util::get_config();

	match &cli.command {
		CliCommands::Script { command, dir } => {
			let scripts_dirname = match dir {
				Some(val) => String::from("scripts-") + val,
				None => String::from("scripts/"),
			};
			let scripts_path = config.dotmgr_dir.clone().join(scripts_dirname);

			match &command {
				ScriptCommands::List {} => {
					std::process::Command::new("ls")
						.args(["-x", scripts_path.to_str().unwrap()])
						.spawn()
						.unwrap();
				}
				ScriptCommands::View { file: script_frag } => {
					let script = util::glob_script(scripts_path.to_str().unwrap(), script_frag);

					Command::new("less")
						.arg(script)
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
				ScriptCommands::Edit { file: script_frag } => {
					let script = util::glob_script(scripts_path.to_str().unwrap(), script_frag);

					Command::new(util::get_editor())
						.args([script])
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
				}
				ScriptCommands::Run {
					file: script_frag,
					sudo,
				} => {
					let get_environment = || -> HashMap<String, String> {
						let s = config.dotmgr_dir.to_str().unwrap();
						let environment_script = String::from(s) + "/impl/environment.sh";

						let mut child = Command::new(environment_script)
							.stdout(Stdio::piped())
							.spawn()
							.unwrap();

						let mut map = HashMap::new();

						if let Some(stdout) = &mut child.stdout {
							let lines = BufReader::new(stdout).lines();
							for maybe_line in lines {
								let line = maybe_line.unwrap();

								match line.find("=") {
									Some(val) => {
										let key = &line.as_str()[..val];
										let value = &line.as_str()[val + 1..];

										map.insert(String::from(key), String::from(value));
									}
									None => {}
								}
							}
						} else {
							panic!("Something went wrong");
						}

						map
					};

					let entrypoint = get_entrypoint(config.dotmgr_dir.to_str().unwrap().clone());
					let script = util::glob_script(scripts_path.to_str().unwrap(), script_frag);
					let sources = util::get_sources(config.dotmgr_dir.to_str().unwrap().clone());

					Command::new(entrypoint)
						.arg(script)
						.arg(sources)
						.envs(get_environment())
						.spawn()
						.unwrap()
						.wait()
						.unwrap();
					// setup terminal
					// terminal::enable_raw_mode().unwrap();
					// let mut stdout = io::stdout();
					// execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
					// let backend = CrosstermBackend::new(stdout);
					// let mut terminal = Terminal::new(backend).unwrap();

					// // run
					// let app = App::new();
					// let res = run_app(&mut terminal, app);

					// // restore terminal
					// terminal::disable_raw_mode().unwrap();
					// execute!(
					// 	terminal.backend_mut(),
					// 	LeaveAlternateScreen,
					// 	DisableMouseCapture
					// )
					// .unwrap();
					// terminal.show_cursor().unwrap();

					// if let Err(err) = res {
					// 	println!("{:?}", err)
					// }

					// // functions
					// fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
					// 	loop {
					// 		terminal.draw(|f| ui(f, &app))?;

					// 		if let Event::Key(key) = event::read()? {
					// 			if let KeyCode::Char('q') = key.code {
					// 				return Ok(());
					// 			}
					// 		}
					// 	}
					// }

					// fn create_block(title: &str) -> Block {
					// 	Block::default()
					// 		.borders(Borders::ALL)
					// 		.style(
					// 			Style::default()
					// 				.bg(tui::style::Color::White)
					// 				.fg(tui::style::Color::Black),
					// 		)
					// 		.title(Span::styled(
					// 			title,
					// 			Style::default().add_modifier(Modifier::BOLD),
					// 		))
					// }

					// fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
					// 	let chunks = Layout::default()
					// 		.direction(Direction::Horizontal)
					// 		.constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
					// 		.split(f.size());

					// 	let block = Block::default().title("Files").borders(Borders::ALL);

					// 	// f.render_widget(block, chunks[0]);
					// 	let text: Vec<Spans> = vec![Spans::from(app.desc_num.to_string())];
					// 	// let mut state = Paragraph::St
					// 	let paragraph = Paragraph::new(text.clone()).block(create_block("other"));
					// 	f.render_widget(paragraph, chunks[0]);

					// 	//
					// 	let block = Block::default()
					// 		.title("Description")
					// 		.borders(Borders::ALL)
					// 		.style(Style::default().add_modifier(Modifier::BOLD));

					// 	f.render_widget(block, chunks[1]);
					// }
				}
			}
		}
		CliCommands::Reconcile { .. } => {
			println!("Not implemented");
		}
		CliCommands::Generate {} => {
			println!("Not implemented");
		}
		CliCommands::Update {} => {
			util::run_hook(&config, When::Before, "update");

			let dir = config.dotmgr_src_dir.to_str().unwrap();
			Command::new("git")
				.args(["-C", dir, "status", "--short"])
				.spawn()
				.unwrap()
				.wait()
				.unwrap();
			Command::new("git")
				.args(["-C", dir, "pull"])
				.spawn()
				.unwrap()
				.wait()
				.unwrap();
			Command::new("git")
				.args(["-C", dir, "status", "--short"])
				.spawn()
				.unwrap()
				.wait()
				.unwrap();

			util::run_hook(&config, When::After, "update");
		}
	}
}
