# shellcheck shell=bash

dotmgr-action() {
	local files=(
		# Common
		'10-idempotent'
		'11-backup'
		''
		# Bootstrapping
		'20-install_packages'
		'21-install_others'
		'22-update_others'
		''
		# regular uncommon
		'31-secrets_export'
		'32-secrets_import'
		'33-_ImportAllVirtualBox'
		'34-_minecraft-sync'
	)
	local action="$1"
	if ! shift; then core.print_error 'Failed to shift'; fi

	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	_helper.source_utils "$user_dotmgr_dir" "$@"
	_helper.run_hook "$user_dotmgr_dir" 'actionBefore' "$@"
	_helper.run_actions "$user_dotmgr_dir/actions" "$action" "$@"
	_helper.run_hook "$user_dotmgr_dir" 'actionAfter' "$@"
}


