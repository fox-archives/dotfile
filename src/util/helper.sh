# shellcheck shell=bash

_helper.run_hook() {
	local user_dotmgr_dir="$1"
	local hook_name="$2"

	local hook_file="$user_dotmgr_dir/hooks/$hook_name.sh"
	if [ -f "$hook_file" ]; then
		source "$hook_file"
	fi
}

_helper.run_actions() {
	local actions_dir="$1"
	local action="$2"

	if [ -n "$action" ]; then
		if [ -f "$actions_dir/$action.sh" ]; then
			source "$actions_dir/$action.sh"
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
		done < "$actions_dir/$file.sh"; unset -v line
	done; unset -v file

	_tty.fullscreen_init
	core.trap_add _tty.fullscreen_deinit_and_exit 'EXIT'
	while :; do
		_helper.private.print_menu "$selected" 'actions' 'descriptions'

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
			"$EDITOR" "$actions_dir/${files[$selected]}.sh"
			;;
		$'\n'|$'\x0d')
			_tty.fullscreen_deinit
			source "$actions_dir/${files[$selected]}.sh"
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
			core.trap_remove _tty.fullscreen_deinit_and_exit 'EXIT'
			_tty.fullscreen_deinit_and_exit
			;;
		esac
	done
}

_helper.private.print_menu() {
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
