# shellcheck shell=bash

dotmgr-doctor() {
	_util.get_user_dotmgr_dir --no-exit
	local user_dotmgr_dir="$REPLY"

	_util.get_user_profile "$user_dotmgr_dir" --no-exit
	local user_profile="$REPLY"

	_helper.source_utils "$user_dotmgr_dir" "$@"
	_helper.run_hook "$user_dotmgr_dir" 'doctorBefore' "$@"
	printf '%s\n' "User Dotmgr Dir: $user_dotmgr_dir"
	printf '%s\n' "User Profile: $user_profile"
	_helper.run_hook "$user_dotmgr_dir" 'doctorAfter' "$@"
}
