# Guide

## Summary

You can customize the behavior of `dotmgr` in three ways

- [Actions](##Actions)
- [Hooks](##Hooks)
- [Profiles](##Profiles)

For each of those three ways, you can use other files or functions

- [Extras](##Extras)
- [Util](##Util)

- TODO: mention `core.*`
- TODO: `DOTMGR_DIR`

## Actions

Actions are at the core of your dotfile management. They are essentially shell scripts, but `dotmgr` parses their documentation and ordering to create a TUI interface to select a particular script super easily

## Hooks

Hooks are placed in the `hooks` subdirectory. The supported hooks are:

- `actionPlumbingBefore.sh`
- `actionPlumbingAfter.sh`
- `actionBefore.sh`
- `actionAfter.sh`
- `bootstrapBefore.sh`
- `bootstrapAfter.sh`
- `doctorBefore.sh`
- `doctorAfter.sh`
- `updateBefore.sh`
- `updateAfter.sh`

The body of the hook must be within the `main()` function. `dotmgr` will source your [utility](##Utilities) files before calling `main()`. Example:

```sh
# shellcheck shell=bash

main() {
	printf '%s\n' 'Hook called!'
}
```

## Profiles

Profiles are used to detect and categorize the currently running system. For example, you might have "server", "desktop", and "laptop" profiles so you can easily deploy different dotfiles

## Extras

Create auxillary files under the `extras` subdirectory. For example, particular Perl script, or a JSON configuration file may live here

## Utilities

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
