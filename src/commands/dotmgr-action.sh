# shellcheck shell=bash

dotmgr-action() {
	local action="$1"

	if [ -n "$action" ]; then
		if [ -f "$DOTMGR_ROOT/src/actions/$action.sh" ]; then
			source "$DOTMGR_ROOT/src/actions/$action.sh"
			if ! shift; then
				core.print_die 'Failed to shift'
			fi
			if ! action "$@"; then
				core.print_die "Failed to execute action"
			fi
		else
			core.print_die "Could not find action '$action'"
		fi
		exit
	fi

	local left_str='                     |'

	local -i selected=0
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

	local -a actions=() descriptions=()
	local file=
	for file in "${files[@]}"; do
		if [ -z "$file" ]; then
			actions+=('')
			descriptions+=('')
			continue
		fi

		local descriptionsi=0
		local mode='default' description=$'\033[0;0H'
		local line=
		while IFS= read -r line; do
			if [ "$line" = '# Name:' ]; then
				mode='name'
				continue
			elif [ "$line" = '# Description:' ]; then
				descriptionsi=0
				mode='description'
				continue
			fi

			if [ "$mode" = 'name' ]; then
				actions+=("${line:2}")
				mode='default'
			elif [ "$mode" = 'description' ]; then
				if [ "${line:0:1}" != '#' ]; then
					descriptions+=("$description")
					mode='default'
					continue
				fi

				local esc=
				printf -v esc '\033[%sC' $((${#left_str}+1))

				if [ -z "${line:2}" ]; then
					description+=$'\n'
				else
					description+="$esc""${line:2}"$'\n'
				fi

				descriptionsi=$((descriptionsi++))
			fi
		done < "$DOTMGR_ROOT/src/actions/$file.sh"; unset -v line
	done; unset -v file

	tty.fullscreen_init
	core.trap_add tty.fullscreen_deinit_and_exit 'EXIT'
	while :; do
		print_menu "$selected" 'actions' 'descriptions'

		local key=
		if ! read -rsN1 key; then
			core.print_die 'Failed to read input'
		fi

		case $key in
		j)
			if ((selected < ${#files[@]} - 1)); then
				if [ -z "${actions[$selected+1]}" ]; then
					selected=$((selected+2))
				else
					((++selected))
				fi
			fi
			;;
		k)
			if ((selected > 0)); then
				if [ -z "${actions[$selected-1]}" ]; then
					selected=$((selected-2))
				else
					((selected--))
				fi
			fi
			;;
		e)
			"$EDITOR" "$DOTMGR_ROOT/src/actions/${files[$selected]}.sh"
			;;
		$'\n'|$'\x0d')
			tty.fullscreen_deinit
			source "$DOTMGR_ROOT/src/actions/${files[$selected]}.sh"
			if ! action; then
				core.print_die "Failed to execute action"
			fi
			exit
			;;
		1|2|3|4|5|6|7|8|9)
			# shellcheck disable=SC1007
			local -i i= adjustedi=-1
			for ((i=0; i < key; ++i)); do
				if [ -z "${files[$i]}" ]; then
					adjustedi=$((adjustedi+1))
				fi

				adjustedi=$((adjustedi+1))
			done
			selected=$adjustedi
			;;
		q) break ;;
		$'\x1b')
			core.trap_remove tty.fullscreen_deinit_and_exit 'EXIT'
			tty.fullscreen_deinit_and_exit
			;;
		esac
	done
}

tty.fullscreen_init() {
	stty -echo
	tput smcup  # save screen contents
	tput civis 2>/dev/null # cursor to invisible

	printf '\033[3J' # clear
	read -r global_tty_height global_tty_width < <(stty size)
}

tty.fullscreen_deinit() {
	tput sgr0
	tput cnorm # cursor to normal
	tput rmcup # restore screen contents
	stty echo
}

tty.fullscreen_deinit_and_exit() {
	tty.fullscreen_deinit
	exit
}

print_menu() {
	# print margin
	local i=
	for ((i = 0; i < global_tty_height; ++i)); do
		printf '\033[%d;%dH' "$i" 0 # tput cup 0 0
		printf "%s" "$left_str"
	done; unset -v i
	printf '\033[%d;%dH' "$global_tty_height" 0
	printf 'q: quit    j: Down    k: Up    Enter: Select    e: Edit'

	# print actions
	printf '\033[%d;%dH' 0 0 # tput cup 0 0
	local i= reali=0
	for ((i=0; i<${#actions[@]}; ++i)); do
		if [ -z "${actions[$i]}" ]; then
			printf '\n'
			continue
		fi

		if ((i == selected)); then
			printf '\033[0;34m'
		fi

		printf '%s\033[0m\n' "$((reali+1)): ${actions[$i]}"
		((++reali))
	done; unset -v i reali

	# reset description
	local i=
	for ((i=0; i<${#actions[@]}; ++i)); do
		printf '\033[%d;%dH' "$i" $((${#left_str}+1)) # tput cup
		printf '\033[K' # tput el
	done; unset -v i

	# print description
	# shellcheck disable=SC2059
	printf "${descriptions[$selected]}"
}

find_mnt_usb() {
	local usb_partition_uuid="$1"

	local block_dev="/dev/disk/by-uuid/$usb_partition_uuid"
	if [ ! -e "$block_dev" ]; then
		core.print_die "USB Not plugged in"
	fi

	local block_dev_target=
	if ! block_dev_target=$(findmnt -no TARGET "$block_dev"); then
		# 'findmnt' exits failure if cannot find block device. We account
		# for that case with '[ -z "$block_dev_target" ]' below
		:
	fi

	# If the USB is not already mounted
	if [ -z "$block_dev_target" ]; then
		if mountpoint -q /mnt; then
			core.print_die "Directory '/mnt' must not already be a mountpoint"
		fi

		util.run sudo mount "$block_dev" /mnt

		if ! block_dev_target=$(findmnt -no TARGET "$block_dev"); then
			core.print_die "Automount failed"
		fi
	fi

	REPLY=$block_dev_target
}