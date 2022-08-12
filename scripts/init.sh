#!/usr/bin/env bash
set -eo pipefail

die() {
	printf '%s\n' "$1. Exiting" >&2
	exit 1
}

ask() {
	unset -v REPLY; REPLY=
	local prompt="$1"
	local defaultText="$2"
	local hintsArrayName="$3"

	printf '%s\n' "Question: $prompt"
	if [ -n "$hintsArrayName" ]; then
		local -n hintsArray="$hintsArrayName"
		local hint=
		for hint in "${hintsArray[@]}"; do
			printf '%s\n' "Hint: $hint"
		done
	fi
	read -rei "$defaultText" -p '> '
	printf '\n'
}

run() {
	"$@" || die "Failed to run command: $*"
}

main() {
	printf '%s\n' "--- Initialization for dotmgr ---"

	# shellcheck disable=SC2034,SC2016
	local hints=(
		'If the directory does not exist, one will be created for you'
		'Using environment variables like "$HOME" is not supported'
	)
	# shellcheck disable=SC2088
	ask "Where are your dotfiles located?" '~/.dotfiles' 'hints'
	local dotfileDir="$REPLY"
	if [[ $dotfileDir == '~'* ]]; then
		dotfileDir="${HOME}${dotfileDir:1}"
	fi

	run mkdir -p "$dotfileDir" # Separate step in case of permission error, etc.
	run mkdir -p "$dotfileDir"/dotmgr/{actions,actions-plumbing,actions-plumbing-sudo,actions-sudo,extras,hooks,profiles}
	run touch "$dotfileDir/bootstrap.sh"

	# Write sample actions
	cat >> "$dotfileDir/dotmgr/actions/10-bootstrap.sh" <<"EOF"
# shellcheck shell=bash

main() {
	printf '%s\n' "Installing various packages and utilities"

	# DEBIAN_FRONTEND=noninteractive sudo apt-get -y --no-install-recommends install \
	# 	apt-transport-https clang

	# NONINTERACTIVE=1 brew install autoenv
}
EOF
	cat > "$dotfileDir/dotmgr/actions/11-dotmgr-say-hello.sh" <<"EOF"
# shellcheck shell=bash

# Name:
# Say Hello
#
# Description:
# Are you a human and wish to give a grand salutation? Well, "hello"
# might just be your word!

main() {
	printf '%s\n' 'Hello!'
}
EOF
	cat > "$dotfileDir/dotmgr/actions/12-dotmgr-say-woof.sh" <<"EOF"
# shellcheck shell=bash

# Name:
# Say Woof
#
# Description:
# Are you a doggy and wish to communicate with your human? If so,
# use this command!

main() {
	printf '%s\n' 'WOOF ^w^'
}
EOF
	cat > "$dotfileDir/dotmgr/actions/20-dotmgr-say-meow.sh" <<"EOF"
# shellcheck shell=bash

# Name:
# Say Meow
#
# Description:
# Are you a cat?

main() {
	printf '%s\n' 'meow'
}
EOF

	# Write sample profiles
	cat >> "$dotfileDir/dotmgr/profiles/0-default.sh" <<"EOF"
# shellcheck shell=bash

main.check() {
	# I personally prefer erroring if neither of my owner profiles are detected
	# That way, I don't accidentally run the wrong commands
	return 1
}
EOF
	cat >> "$dotfileDir/dotmgr/profiles/1-desktop.sh" <<"EOF"
# shellcheck shell=bash

main.check() {
	# If the current system is a Desktop (3) system, returns true; otherwise, returns false
	if [ "$(</sys/class/dmi/id/chassis_type)" = '3' ]; then :; else
		return $?
	fi
}

main.vars() {
	VAR_REPO_DIR="$HOME/repos"
}
EOF

	# Write sample bootstrap
	cat >> "$dotfileDir/dotmgr/bootstrap.sh" <<"_EOF"
#!/usr/bin/env bash
set -eo pipefail

if [ "${BASH_SOURCE[0]}" != "$0" ]; then
	printf '%s\n' "Error: This file should not be sourced"
	return 1
fi

git clone 'https://github.com/hyperupcall/dotmgr' ~/.bootstrap/dotmgr

# When this file is sourced, we want `dotmgr` to be in the `PATH`. This is a temporary thing, that
# only affects a single shell. Make sure you make it a perminant thing when `dotmgr bootstrap` is
# ran (by modifying the PATH in a `~/.dotfiles/hooks/bootstrapAfter.sh` file)
cat > ~/.bootstrap/bootstrap-out.sh <<"EOF"
PATH="$HOME/.bootstrap/dotmgr/bin:$PATH"
EOF

# Remind the user what to do by printing to the console
cat <<"EOF"
--- Run the following ---
source ~/.bootstrap/bootstrap-out.sh
dotmgr bootstrap
EOF
_EOF
	run chmod +x "$dotfileDir/dotmgr/bootstrap.sh"
}

main "$@"
