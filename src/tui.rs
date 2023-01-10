use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use glob::glob;
use std::{
	io::{self},
	path::{Path, PathBuf},
};
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Constraint, Direction, Layout},
	style::{Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, Borders, List, ListItem, Paragraph},
	Frame, Terminal,
};

enum Selected {
	Left,
	Right,
}

struct App {
	current_tab: Selected,
	dir: PathBuf,
	current_char: char,
	file_num: u32,
	desc_num: u32,
}

impl App {
	fn new(dir: PathBuf) -> App {
		App {
			current_tab: Selected::Left,
			dir,
			current_char: '?',
			file_num: 1,
			desc_num: 2,
		}
	}
}

pub fn show_menu(dir: PathBuf) -> PathBuf {
	// setup terminal
	terminal::enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend).unwrap();

	// run
	let app = App::new(dir);
	let res = run_app(&mut terminal, app);

	// restore terminal
	terminal::disable_raw_mode().unwrap();
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)
	.unwrap();
	terminal.show_cursor().unwrap();

	if let Err(err) = res {
		println!("{:?}", err)
	}

	PathBuf::from("whatever")
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
	loop {
		terminal.draw(|f| ui(f, &app))?;

		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Char('h') => app.current_char = 'h',
				KeyCode::Left => app.current_char = 'h',
				KeyCode::Char('j') => app.current_char = 'j',
				KeyCode::Down => app.current_char = 'j',
				KeyCode::Char('l') => app.current_char = 'l',
				KeyCode::Right => app.current_char = 'l',
				KeyCode::Char('k') => app.current_char = 'k',
				KeyCode::Up => app.current_char = 'k',
				KeyCode::Char('q') => return Ok(()),
				KeyCode::Tab => {
					app.current_tab = match app.current_tab {
						Selected::Left => Selected::Right,
						Selected::Right => Selected::Left,
					}
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

	//
	let mut app_dir = String::from(app.dir.to_str().unwrap());
	app_dir.push_str("/*.");
	app_dir.push_str("sh");

	let items: Vec<ListItem> = glob(app_dir.as_str())
		.unwrap()
		.map(|item| ListItem::new(item.unwrap().to_str().unwrap().to_owned()).style(Style::default()))
		.collect();

	let mut b = create_block("Files");
	match app.current_tab {
		Selected::Left => {
			b = b.style(Style::default().add_modifier(Modifier::BOLD));
		}
		_ => {}
	}
	let w = List::new(items).block(b);
	f.render_widget(w, chunks[0]);

	// let block = Block::default().title("Files").borders(Borders::ALL);

	// // f.render_widget(block, chunks[0]);
	// let text: Vec<Spans> = vec![Spans::from(app.desc_num.to_string())];
	// // let mut state = Paragraph::St
	let text = "Stuff";
	let mut b = create_block("Content");
	match app.current_tab {
		Selected::Right => {
			b = b.style(Style::default().add_modifier(Modifier::BOLD));
		}
		_ => {}
	}
	let w = Paragraph::new(text.clone()).block(b);
	f.render_widget(w, chunks[1]);
}

fn create_block(title: &str) -> Block {
	Block::default()
		.borders(Borders::ALL)
		.style(Style::default())
	// z
}

// pub fn do_things() {
// 	// setup terminal
// 	terminal::enable_raw_mode().unwrap();
// 	let mut stdout = io::stdout();
// 	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
// 	let backend = CrosstermBackend::new(stdout);
// 	let mut terminal = Terminal::new(backend).unwrap();

// 	// run
// 	let app = App::new();
// 	let res = run_app(&mut terminal, app);

// 	// restore terminal
// 	terminal::disable_raw_mode().unwrap();
// 	execute!(
// 		terminal.backend_mut(),
// 		LeaveAlternateScreen,
// 		DisableMouseCapture
// 	)
// 	.unwrap();
// 	terminal.show_cursor().unwrap();

// 	if let Err(err) = res {
// 		println!("{:?}", err)
// 	}

// 	// functions
// 	fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
// 		loop {
// 			terminal.draw(|f| ui(f, &app))?;

// 			if let Event::Key(key) = event::read()? {
// 				if let KeyCode::Char('q') = key.code {
// 					return Ok(());
// 				}
// 			}
// 		}
// 	}

// 	fn create_block(title: &str) -> Block {
// 		Block::default()
// 			.borders(Borders::ALL)
// 			.style(
// 				Style::default()
// 					.bg(tui::style::Color::White)
// 					.fg(tui::style::Color::Black),
// 			)
// 			.title(Span::styled(
// 				title,
// 				Style::default().add_modifier(Modifier::BOLD),
// 			))
// 	}

// 	fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
// 		let chunks = Layout::default()
// 			.direction(Direction::Horizontal)
// 			.constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
// 			.split(f.size());

// 		let block = Block::default().title("Files").borders(Borders::ALL);

// 		// f.render_widget(block, chunks[0]);
// 		let text: Vec<Spans> = vec![Spans::from(app.desc_num.to_string())];
// 		// let mut state = Paragraph::St
// 		let paragraph = Paragraph::new(text.clone()).block(create_block("other"));
// 		f.render_widget(paragraph, chunks[0]);

// 		//
// 		let block = Block::default()
// 			.title("Description")
// 			.borders(Borders::ALL)
// 			.style(Style::default().add_modifier(Modifier::BOLD));

// 		f.render_widget(block, chunks[1]);
// 	}
// }
