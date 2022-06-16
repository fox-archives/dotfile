# shellcheck shell=bash

dotmgrUtil.ensure_profile_read() {
	local has_found_profile='no'
	for profile_file in  "$DOTMGR_ROOT/src/profiles"/?*.sh; do
		source "$profile_file"
		local profile_name="${profile_file##*/}"; profile_name=${profile_name%.sh}

		if "$profile_name".check; then
			"$profile_name".vars
			has_found_profile='yes'
			break
		fi
	done

	if [ "$has_found_profile" = 'no' ]; then
		core.print_die 'No matching profile could be found'
	fi
}

dotmgrUtil.prereq() {
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

dotmgrUtil.show_help() {
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
