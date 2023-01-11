use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use glob::glob;
use std::{
	io::{self},
	path::PathBuf,
	process::exit,
};
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Constraint, Direction, Layout},
	style::{Modifier, Style},
	text::Span,
	widgets::{Block, Borders, List, ListItem},
	Frame, Terminal,
};

enum Selected {
	Left,
	Right,
}

struct App {
	active_pane: Selected,
	path_dir: PathBuf,
	dirnames: Vec<String>,
	selected_file_index: usize,
	viewer_content: String,
}

impl App {
	fn new(dir: PathBuf, dirnames: Vec<String>) -> App {
		App {
			active_pane: Selected::Left,
			path_dir: dir,
			dirnames,
			selected_file_index: 0,
			viewer_content: String::default(),
		}
	}
}

pub fn choose_script(dir: PathBuf) -> PathBuf {
	// setup terminal
	terminal::enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend).unwrap();

	// run
	let app_dir = dir.join("*.sh");
	let files: Vec<String> = glob(app_dir.to_str().unwrap())
		.unwrap()
		.map(|item| {
			let pbuf = item.unwrap();
			let dirname = pbuf.file_name().unwrap();
			String::from(dirname.to_str().unwrap())
		})
		.collect();
	let app = App::new(dir, files);
	let res = run_app(&mut terminal, app);
	let option = res.unwrap();

	// restore terminal
	terminal::disable_raw_mode().unwrap();
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)
	.unwrap();
	terminal.show_cursor().unwrap();

	match option {
		Some(val) => val,
		None => exit(0),
	}
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<Option<PathBuf>> {
	update_viewer(&mut app);

	loop {
		terminal.draw(|f| ui(f, &app))?;

		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Char('q') => return Ok(Option::None),
				KeyCode::Char('k') => {
					if app.selected_file_index > 0 {
						app.selected_file_index -= 1
					}
					update_viewer(&mut app)
				}
				KeyCode::Char('j') => {
					if app.selected_file_index < app.dirnames.len() - 1 {
						app.selected_file_index += 1
					}
					update_viewer(&mut app)
				}
				KeyCode::Tab => {
					app.active_pane = match app.active_pane {
						Selected::Left => Selected::Right,
						Selected::Right => Selected::Left,
					}
				}
				KeyCode::Enter => {
					let dirname = app.dirnames[app.selected_file_index].as_str();
					let fullpath = PathBuf::from(app.path_dir.to_str().unwrap()).join(dirname);
					return Ok(Option::Some(fullpath));
				}
				_ => {}
			}
		}
	}
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
		.split(f.size());

	{
		let app_dir = String::from(app.path_dir.to_str().unwrap()) + "/*.sh";
		let files: Vec<ListItem> = glob(&app_dir)
			.unwrap()
			.enumerate()
			.map(|(i, item)| {
				let pbuf = item.unwrap();
				let dirname = pbuf.file_name().unwrap();
				let dirname_str = dirname.to_str().unwrap().to_owned();
				let span = if app.selected_file_index == i {
					Span::styled(dirname_str, Style::default().add_modifier(Modifier::BOLD))
				} else {
					Span::from(dirname_str)
				};
				return ListItem::new(span);
			})
			.collect();

		let title = "Files";
		let span = match app.active_pane {
			Selected::Left => Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
			_ => Span::from(title),
		};
		let block = Block::default().borders(Borders::ALL).title(span);
		let list = List::new(files).block(block);
		f.render_widget(list, chunks[0]);
	}

	{
		let title = "Viewer";
		let span = match app.active_pane {
			Selected::Right => Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
			_ => Span::from(title),
		};
		let block = Block::default().borders(Borders::ALL).title(span);
		let list = List::new(vec![ListItem::new(app.viewer_content.as_str())]).block(block);
		f.render_widget(list, chunks[1]);
	}
}

fn update_viewer(app: &mut App) {
	let dirname = app.dirnames[app.selected_file_index].as_str();
	let fullpath = PathBuf::from(app.path_dir.to_str().unwrap()).join(dirname);
	// let content = fs::read_to_string(fullpath).unwrap();
	app.viewer_content = String::from(fullpath.to_str().unwrap())
}
