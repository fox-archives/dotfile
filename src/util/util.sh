# shellcheck shell=bash

_util.get_user_profile() {
	unset -v REPLY; REPLY=
	local user_dotmgr_dir="$1"

	# shellcheck disable=SC1007
	local arg= flag_no_exit='no'
	for arg; do case $arg in
		--no-exit) flag_no_exit='yes'
	esac done; unset -v arg

	# shellcheck disable=SC1007
	local profile_name= has_found_profile='no'
	for profile_file in  "$user_dotmgr_dir/profiles"/?*.sh; do
		source "$profile_file"
		profile_name=${profile_file##*/}; profile_name=${profile_name#*-}; profile_name=${profile_name%.sh}

		if profile.check; then
			profile.vars
			has_found_profile='yes'
			break
		fi
	done

	if [[ $has_found_profile == 'no' && $flag_no_exit == 'no' ]]; then
		core.print_die 'No matching profile could be found'
	fi

	REPLY="$profile_name"
}

_util.get_user_dotmgr_dir() {
	unset -v REPLY; REPLY=

	# shellcheck disable=SC1007
	local arg= flag_no_exit='no'
	for arg; do case $arg in
		--no-exit) flag_no_exit='yes'
	esac done; unset -v arg

	if [[ -v DOTMGR_DIR ]]; then
		REPLY="$DOTMGR_DIR"
		return
	fi

	if [ -f "$DOTMGR_ROOT/.dotmgr_dir" ]; then
		local dir=
		dir=$(<"$DOTMGR_ROOT/.dotmgr_dir")
		if [[ $dir == '~'* ]]; then
			dir="${HOME}${dir:1}"
		fi
		REPLY=$dir
	elif [ -f "$HOME/.dotfiles/dotmgr" ]; then
		REPLY="$HOME/.dotfiles/dotmgr"
	elif [ -f "$HOME/.dots/dotmgr" ]; then
		REPLY="$HOME/.dots/dotmgr"
	else
		if [ "$flag_no_exit" = 'no' ]; then
			core.print_die "Failed to find your dotmgr directory"
		fi
	fi
}

_util.get_file_list() {
	unset -v REPLY; declare -g REPLY=()
	local actions_dir="$1"

	# shellcheck disable=SC1007
	local file= previous_section_number=
	for file in "$actions_dir"/{0..9}*.sh; do
		local file_name="${file##*/}"
		local section_number="${file_name::1}"

		if [[ -n "$previous_section_number" && "$section_number" != "$previous_section_number" ]]; then
			REPLY+=('')
		fi

		REPLY+=("${file_name%.sh}")
		previous_section_number=$section_number
	done; unset -v file
}

_util.get_action_file() {
	unset -v REPLY; REPLY=
	local action_dir="$1"
	local action="$2"

	if [ -z "$action" ]; then
		REPLY=
		return
	fi

	core.shopt_push -s nullglob
	local -a action_files=("$action_dir"/*"$action"*)
	core.shopt_pop

	if (( ${#action_files[@]} == 0 )); then
		core.print_die "Failed to find file matching '$action'"
	elif (( ${#action_files[@]} > 1 )); then
		core.print_die "More than one file was matched with action: '$action'"
	fi

	REPLY=${action_files[0]}
}

_util.source_and_run_main() {
	local file="$1"

	if ! shift; then
		core.panic 'Failed to shift'
	fi

	unset -f main
	source "$file"
	if declare -f main &>/dev/null; then
		if DOTFILES_ROOT="${user_dotmgr_dir%/*}" main "$@"; then :; else
			return $?
		fi
	else
		core.print_die "File '$file' does not have a main() function"
	fi
}

_util.show_help() {
	cat <<-EOF
		Usage:
		  dotmgr [command]

		Commands:
		  action [--list] [--view] [--edit] [--sudo] [file]
		    Perform a particular action. If no action was given, show
		    a selection screen for the different actions

		  action-plumbing [--list] [--view] [--edit] [--sudo] [file]
		    Perform a plumbing action. These are automatically called by 'action', but
		    in case of issues, they can be called manually

		  doctor
		    Get information about the current system. Currently, it lists
		    information about the current profile

		  update
		    Updates dotmgr to the latest release

		Flags:
		  --help
		    Show help menu

		Examples:
		  dotmgr action
	EOF
}
