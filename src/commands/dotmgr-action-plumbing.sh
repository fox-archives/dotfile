# shellcheck shell=bash

dotmgr-action-plumbing() {
	_helper.parse_action_args --internal-plumbing "$@"
	local action_dir="$REPLY1"
	local action_file="$REPLY2"

	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	_helper.source_utils "$user_dotmgr_dir" "$@"
	if ((EUID == 0)); then
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingBeforeSudo' "$@"
		_helper.run_actions "$action_dir" "$action_file" "$@"
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingAfterSudo' "$@"
	else
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingBefore' "$@"
		_helper.run_actions "$action_dir" "$action_file" "$@"
		_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingAfter' "$@"
	fi
}
