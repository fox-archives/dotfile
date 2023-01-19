# Guide

- [Guide](#guide)
  - [Summary](#summary)
  - [Scripts](#scripts)
    - [Utilities](#utilities)
  - [impl](#impl)
  - [Extras](#extras)

## Summary

You can customize the behavior of `dotmgr` in three ways

## Scripts

Scripts, located in `scripts` or `scripts-*` are at the core of your dotfile management. They are shell scripts, but `dotmgr` parses their documentation and ordering to create a TUI interface to select a particular script super easily

For every script, use `util` to put common functions for every script to use

### Utilities

Create utility and helper functions under the `util` subdirectory.

Simply place your functions within a file with a `.sh` file ending.

For example, the following can be put in a `util/dot.sh` file:

```sh
# shellcheck shell=bash

dot.install_cmd() {
	local cmd="$1"
	local pkg="$2"

	if iscmd "$cmd"; then
		log "Already installed $cmd"
	else
		log "Installing $cmd"

		if iscmd 'pacman'; then
			run sudo pacman -S --noconfirm "$pkg"
		elif iscmd 'apt-get'; then
			run sudo apt-get -y install "$pkg"
		elif iscmd 'dnf'; then
			run sudo dnf -y install "$pkg"
		elif iscmd 'zypper'; then
			run sudo zypper -y install "$pkg"
		elif iscmd 'eopkg'; then
			run sudo eopkg -y install "$pkg"
		elif iscmd 'brew'; then
			run brew install "$pkg"
		fi

		if ! iscmd "$cmd"; then
			die "Automatic installation of $cmd failed"
		fi
	fi
}
```

Now, your function is callable by any of your hooks, actions, or profiles like so:

```sh
dot.install_cmd 'nvim' 'neovim'
```

## impl

Write `entrypoint.*` to write an entrypoint for a particular language

Write `environment.sh` to print out an environment to be parsed and used for execution of any script.

## Extras

Create auxillary files under the `extras` subdirectory. For example, a particular Perl script, or a JSON configuration file may live here. This isn't used by dotmgr directly, but it's a convention.
