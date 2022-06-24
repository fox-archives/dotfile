# shellcheck shell=bash

main.check() {
	# I personally prefer erroring if neither of my owner profiles are detected
	# That way, I don't accidentally run the wrong commands
	return 1
}
