# shellcheck shell=bash

# Assumptions:
# sudo, git, nvim installed
# hyperupcall/dots cloned
# dotmgr in PATH

dotmgr-bootstrap() {
	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	_helper.run_hook "$user_dotmgr_dir" 'bootstrapBefore'

	# Bootstraps are inherently bespoke, so code intentionally ommited here

	_helper.run_hook "$user_dotmgr_dir" 'bootstrapAfter'
}
