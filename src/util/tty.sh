# shellcheck shell=bash

_tty.fullscreen_init() {
	stty -echo
	tput smcup  # save screen contents
	tput civis 2>/dev/null # cursor to invisible

	printf '\033[3J' # clear
	read -r global_tty_height global_tty_width < <(stty size)
}

_tty.fullscreen_deinit() {
	tput sgr0
	tput cnorm # cursor to normal
	tput rmcup # restore screen contents
	stty echo
}

_tty.fullscreen_deinit_and_exit() {
	_tty.fullscreen_deinit
	exit
}
