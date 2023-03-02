use notify::{PollWatcher, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind};
use std::{
	path::{Path, PathBuf},
	process::{exit, Command},
	time::Duration,
};

use crate::config::Config;
use crate::util;

pub struct CommandInternal {
	config: Config,
}

impl CommandInternal {
	pub fn new(config: Config) -> Self {
		Self { config }
	}

	pub fn start_watcher(&self) {}

	pub fn find_man(&self) {}

	pub fn generate(&self) {}
}

pub fn internal_start_watcher() {
	println!("Starting watcher");

	let (tx, rx) = std::sync::mpsc::channel();
	// This example is a little bit misleading as you can just create one Config and use it for all watchers.
	// That way the pollwatcher specific stuff is still configured, if it should be used.
	let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
		// custom config for PollWatcher kind
		// you
		let config = notify::Config::default().with_poll_interval(Duration::from_secs(1));
		Box::new(PollWatcher::new(tx, config).unwrap())
	} else {
		// use default config for everything else
		Box::new(RecommendedWatcher::new(tx, notify::Config::default()).unwrap())
	};

	// watch some stuff
	watcher
		.watch(Path::new("."), RecursiveMode::Recursive)
		.unwrap();

	// just print all events, this blocks forever
	for e in rx {
		println!("{:?}", e);
	}
}

pub fn internal_find_man(command_line: String) {
	let str = util::find_man(command_line);
	println!("{}", str.as_str());
}

pub fn internal_generate() {}
