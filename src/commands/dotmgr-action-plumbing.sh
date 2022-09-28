# shellcheck shell=bash

dotmgr-action-plumbing() {
	local -a pkgs=()
	# shellcheck disable=SC1007
	local arg= flag_sudo='no' flag_list='no'
	for arg; do case $arg in
	--sudo)
		flag_sudo='yes'
		;;
	--list)
		flag_list='yes'
	-*)
		print.die "Flag '$arg' not recognized"
		;;
	*)
		pkgs+=("$arg")
		;;
	esac done; unset -v arg

	local action="${pkgs[0]}"

	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	if [ "$flag_sudo" = 'yes' ] && (( EUID != 0)); then
		DOTMGR_DIR="$user_dotmgr_dir" exec sudo --preserve-env='DOTMGR_DIR' "$0" action-plumbing "$@"
	fi

	if [ "$flag_list" = 'yes' ]; then
		if ((EUID == 0)); then
			ls -- "$user_dotmgr_dir/actions-plumbing-sudo"
		else
			ls -- "$user_dotmgr_dir/actions-plumbing"
		fi

		return
	fi

	_helper.source_utils "$user_dotmgr_dir" "$@"
	if ((EUID == 0)); then
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingBeforeSudo' "$@"
		_helper.run_actions "$user_dotmgr_dir/actions-plumbing-sudo" "$action" "$@"
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingAfterSudo' "$@"
	else
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingBefore' "$@"
		_helper.run_actions "$user_dotmgr_dir/actions-plumbing" "$action" "$@"
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingAfter' "$@"
	fi
}
