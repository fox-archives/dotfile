# shellcheck shell=bash

_util.debug() {
	local msg="$1"

	if [[ -v DEBUG ]]; then
		printf '%s\n' "DEBUG: $msg"
	fi
}

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

_util.prereq() {
	if [ -z "$XDG_CONFIG_HOME" ]; then
		# shellcheck disable=SC2016
		core.print_die '$XDG_CONFIG_HOME is empty. Did you source profile-pre-bootstrap.sh?'
	fi

	if [ -z "$XDG_DATA_HOME" ]; then
		# shellcheck disable=SC2016
		core.print_die '$XDG_DATA_HOME is empty. Did you source profile-pre-bootstrap.sh?'
	fi

	if [ -z "$XDG_STATE_HOME" ]; then
		# shellcheck disable=SC2016
		core.print_die '$XDG_STATE_HOME is empty. Did you source profile-pre-bootstrap.sh?'
	fi
}

_util.show_help() {
	cat <<-EOF
		Usage:
		  dotmgr [command]

		Commands:
		  bootstrap
		    Bootstrap operations that occur before dotfiles have been deployed

		  action [--sudo]
		    Perform a particular action. If no action was given, show
		    a selection screen for the different actions

		  action-plumbing [--sudo]
		    Perform a plumbing action. These are automatically called by 'action', but
		    in case of issues, they can be called manually

		  doctor
		    Get information about the current system. Currently, it lists
		    information about the current profile

		  update
		    Update dotmgr

		  test
		    Search the dotfiles repository for Bats testing files. For each one,
			 change directory and execute Bats

		Flags:
		  --help
		    Show help menu

		Examples:
		  dotmgr bootstrap
		  dotmgr action
	EOF
}
