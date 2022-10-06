# shellcheck shell=bash

dotmgr-action-plumbing() {
	_helper.parse_action_args "$@"

	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

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
