# shellcheck shell=bash

dotmgr-action() {
	_helper.parse_action_args "$@"
	local action_file="$REPLY"

	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	_helper.source_utils "$user_dotmgr_dir" "$@"
	if ((EUID == 0)); then
		_helper.run_hook "$user_dotmgr_dir" 'actionBeforeSudo' "$@"
		_helper.run_actions "$action_file" "$@"
		_helper.run_hook "$user_dotmgr_dir" 'actionAfterSudo' "$@"
	else
		_helper.run_hook "$user_dotmgr_dir" 'actionBefore' "$@"
		_helper.run_actions "$action_file" "$@"
		_helper.run_hook "$user_dotmgr_dir" 'actionAfter' "$@"
	fi
}
