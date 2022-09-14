# shellcheck shell=bash

set -eETo pipefail
shopt -s dotglob extglob globstar nullglob shift_verbose

for f in "$DOTMGR_ROOT"/src/{helpers,util}/?*.sh; do
	if [ "${f##*/}" = "${BASH_SOURCE[1]##*/}" ]; then
		continue
	fi

	source "$f"
done; unset -v f

for f in "$DOTMGR_ROOT"/vendor/bash-core/pkg/src/{public,util}/?*.sh; do
	source "$f"
done; unset -v f

for f in "$DOTMGR_ROOT"/vendor/bash-term/pkg/src/{public,util}/?*.sh; do
	source "$f"
done; unset -v f

global_err_handler() {
	core.print_stacktrace
}
core.trap_add 'global_err_handler' ERR
