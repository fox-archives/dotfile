#!/usr/bin/env bash
set -eo pipefail

if [ "${BASH_SOURCE[0]}" != "$0" ]; then
	printf '%s\n' "Error: This file should not be sourced"
	return 1
fi

git clone 'https://github.com/hyperupcall/dotmgr' ~/.bootstrap/dotmgr

# When this file is sourced, we want `dotmgr` to be in the `PATH`. This is a temporary thing, that
# only affects a single shell. Make sure you make it a permenant thing when `dotmgr action
# bootstrap` is ran
cat > ~/.bootstrap/bootstrap-out.sh <<"EOF"
PATH="$HOME/.bootstrap/dotmgr/bin:$PATH"
EOF

# Remind the user what to do by printing to the console
cat <<"EOF"
--- Run the following ---
source ~/.bootstrap/bootstrap-out.sh
dotmgr action bootstrap
EOF
