# shellcheck shell=bash

main() {
	printf '%s\n' "Installing various packages and utilities"

	# DEBIAN_FRONTEND=noninteractive sudo apt-get -y --no-install-recommends install \
	# 	apt-transport-https clang

	# NONINTERACTIVE=1 brew install autoenv
}
