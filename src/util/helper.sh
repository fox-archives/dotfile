# shellcheck shell=bash

_helper.source_utils() {
	local user_dotmgr_dir="$1"
	if ! shift; then core.print_error 'Failed to shift'; fi

	local f=
	for f in "$user_dotmgr_dir/util"/*.sh; do
		source "$f"
	done; unset -v f
}

_helper.parse_action_args() {
	unset -v REPLY; REPLY=

	local -a pkgs=()
	# shellcheck disable=SC1007
	local arg= flag_{list,view,edit,sudo}='no'
	for arg; do case $arg in
	--list)
		flag_list='yes'
		;;
	--view)
		flag_view='yes'
		;;
	--edit)
		flag_edit='yes'
		;;
	--sudo)
		flag_sudo='yes'
		;;
	-*)
		print.die "Flag '$arg' not recognized"
		;;
	*)
		actions+=("$arg")
		;;
	esac done; unset -v arg

	if ((${#actions[@]} > 1)); then
		core.print_error "Must only pass one action name"
		exit 1
	fi

	_util.get_user_dotmgr_dir
	local user_dotmgr_dir="$REPLY"

	if [ "$flag_sudo" = 'yes' ] && (( EUID != 0)); then
		DOTMGR_DIR="$user_dotmgr_dir" exec sudo --preserve-env='DOTMGR_DIR' "$0" action "$@"
	fi

	local dir=
	if ((EUID == 0)); then
		dir="$user_dotmgr_dir/actions-sudo"
	else
		dir="$user_dotmgr_dir/actions"
	fi

	_util.get_action_file "$dir" "${actions[0]}"
	local action_file="$REPLY"

	if [ "$flag_list" = 'yes' ]; then
		if ((EUID == 0)); then
			ls -- "$dir"
		else
			ls -- "$dir"
		fi

		exit 0
	fi

	if [ "$flag_view" = 'yes' ]; then
		if [ -n "$PAGER" ]; then
			exec "$PAGER" "$action_file"
		fi

		exec less "$action_file"

		exit 0
	fi

	if [ "$flag_edit" = 'yes' ]; then
		if [ -n "$VISUAL" ]; then
			exec "$VISUAL" "$action_file"
		fi

		exec vim "$action_file"

		exit 0
	fi

	REPLY=$action_file
}

_helper.run_hook() {
	local user_dotmgr_dir="$1"
	local hook_name="$2"

	local hook_file="$user_dotmgr_dir/hooks/$hook_name.sh"
	if [ -f "$hook_file" ]; then
		_util.source_and_run_main "$hook_file" "$@"
	fi
}

_helper.run_actions() {
	local actions_dir="$1"
	local action_file="$2"

	_util.get_file_list "$actions_dir"
	local -a files_list=("${REPLY[@]}")

	if [ -n "$action_file" ]; then
		_util.source_and_run_main "$action_file" "$@"
		exit 0
	fi

	local left_str='                       |'
	local -i selected=0

	local -a actions=() descriptions=()
	local file=
	for file in "${files_list[@]}"; do
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
					mode='default-post'
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

		if [[ $mode != 'default' && $mode != 'description' && $mode != 'default-post' ]]; then
			actions+=("$file")

			term.italic 'Not Applicable'
			local text="$REPLY"
			descriptions+=("${description}${esc}${text}"$'\n')
		fi
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
			if ((selected < ${#files_list[@]} - 1)); then
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
			core.trap_remove _tty.fullscreen_deinit_and_exit 'EXIT'
			_tty.fullscreen_deinit
			"$EDITOR" "$actions_dir/${files_list[$selected]}.sh"
			_tty.fullscreen_init
			;;
		$'\n'|$'\x0d')
			core.trap_remove _tty.fullscreen_deinit_and_exit 'EXIT'
			_tty.fullscreen_deinit
			_util.source_and_run_main "$actions_dir/${files_list[$selected]}.sh"
			exit
			;;
		1|2|3|4|5|6|7|8|9)
			# shellcheck disable=SC1007
			local -i i= adjustedi=-1
			for ((i=0; i < key; ++i)); do
				if [ -z "${files_list[$i]}" ]; then
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
