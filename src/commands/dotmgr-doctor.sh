# shellcheck shell=bash

dotmgr-doctor() {
	_util.get_user_dotmgr_dir --no-exit
	local user_dotmgr_dir="$REPLY"

	_util.ensure_profile_read

	_helper.run_hook "$user_dotmgr_dir" 'doctorBefore'
	printf '%s\n' "REPO_DIR_REPLY: $REPO_DIR_REPLY"
	_helper.run_hook "$user_dotmgr_dir" 'doctorAfter'
}
