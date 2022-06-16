# shellcheck shell=bash

main.dotmgr() {
	source "$DOTMGR_ROOT/src/util/source.sh"
	dotmgrUtil.prereq

	local arg=
	for arg; do case $arg in
	-h|--help)
		dotmgrUtil.show_help
		exit
		;;
	esac done; unset -v arg

	local subcommand="$1"
	if [ -f "$DOTMGR_ROOT/src/commands/dotmgr-$subcommand.sh" ]; then
		if ! shift; then
			core.print_die "Failed to shift"
		fi
		source "$DOTMGR_ROOT/src/commands/dotmgr-$subcommand.sh"
		dotmgr-"$subcommand" "$@"
	else
		dotmgrUtil.show_help
		core.print_error 'No matching subcommand found'
		exit 1
	fi
}
