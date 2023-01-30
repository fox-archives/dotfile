use std::{env, fmt, path::PathBuf};

pub struct Config {
	pub dotfiles_dir: PathBuf,
	pub os_dir: PathBuf,
	pub dotmgr_dir: PathBuf,
}

impl Default for Config {
	fn default() -> Config {
		let dotfiles_dir = match env::var("DOTMGR_DOTFILES_DIR") {
			Ok(val) => {
				let p = PathBuf::from(val);
				if p.is_absolute() {
					p
				} else {
					PathBuf::from(dirs::home_dir().unwrap()).join(".dotfiles")
				}
			}
			Err(_) => PathBuf::from(dirs::home_dir().unwrap()).join(".dotfiles"),
		};

		let os_dir = match env::consts::OS {
			"windows" => dotfiles_dir.clone().join("os/windows"),
			_ => dotfiles_dir.clone().join("os/unix"),
		};

		Config {
			dotfiles_dir,
			os_dir: os_dir.clone(),
			dotmgr_dir: os_dir.join("dotmgr"),
		}
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"dotfiles_dir: {}\nos_dir: {},\ndotmgr_dir: {}\n",
			self.dotfiles_dir.to_str().unwrap(),
			self.os_dir.to_str().unwrap(),
			self.dotmgr_dir.to_str().unwrap()
		)
	}
}
