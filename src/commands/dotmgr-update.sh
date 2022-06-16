# shellcheck shell=bash

dotmgr-update() {
	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	_helper.run_hook "$user_dotmgr_dir" 'updateBefore'

	_helper.run_hook "$user_dotmgr_dir" 'updateAfter'
}
