# shellcheck shell=bash

dotmgr-action-plumbing() {
	local action="$1"

	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	_helper.source_utils "$user_dotmgr_dir" "$@"
	_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingBefore' "$@"
	_helper.run_actions "$user_dotmgr_dir/actions-plumbing" "$action" "$@"
	_helper.run_hook "$user_dotmgr_dir" 'actionPlumbingAfter' "$@"
}
