# Utility

# @description From start to current cursor, get the array of subcommands
_dotmgr_wowzers_get_fn() {
	unset -v REPLY
	REPLY=_dotmgr_cmd

	local word=
	for word in "${COMP_WORDS[@]}"; do
		case $word in
			-*) ;;
			*) REPLY+=${word:+::$word}
		esac
	done; unset -v word
}

_dotmgr_wowzers_util_next_subcommand() {
	:
}

# Custom
_dotmgr_cmd::dotmgr() {
	local cur="${COMP_WORDS[COMP_CWORD]}"
	COMPREPLY=($(compgen -W "run doctor update --help" -- "$cur"))
}

_dotmgr_cmd::dotmgr::run() {
	local cur="${COMP_WORDS[COMP_CWORD]}"

	local mode='MODE_DEFAULT'
	echo "$cur" >> /tmp/f
	if [[ $cur == -d* ]]; then
		mode='MODE_COMPLETE_DIR'
	fi

	if [ "$mode" = 'MODE_DEFAULT' ]; then
		local dir=
		local word=
		for word in "${COMP_WORDS[@]}"; do case $word in
		-d=*)
			dir=${word#*=}
			;;
		esac; done

		local -a scripts=()
		local look_dir="$dotmgr_dir/run"
		if [ -n "$dir" ]; then
			look_dir="$look_dir-$dir"
		fi
		shopt -s nullglob
		scripts+=("$look_dir"/*)
		scripts=("${scripts[@]##*/}")
		shopt -u nullglob

		COMPREPLY=($(compgen -W "--list --view --edit --sudo -d ${scripts[*]}" -- "$cur"))
	elif [ "$mode" = 'MODE_COMPLETE_DIR' ]; then
		local -a runs=()
		shopt -s nullglob
		runs+=("$dotmgr_dir"/run-*)
		runs=("${runs[@]##*/}")
		runs=("${runs[@]#run-}")
		shopt -u nullglob

		runs=("${runs[@]/#/-d=}")

		COMPREPLY=($(compgen -W "${runs[*]}" -- "$cur"))
	fi


}

_dotmgr_cmd::dotmgr::doctor() {
	COMPREPLY=()
}

_dotmgr_cmd::dotmgr::update() {
	COMPREPLY=()
}

_dotmgr() {
	# FIXME do not hardcode
	dotmgr_dir="$HOME/.dots/dotmgr"
	# COMP_WORDBREAKS=$' \n"\'><=;|&(:' # default
	COMP_WORDBREAKS=$' \n"\'><;|&(:' # default

	_dotmgr_wowzers_get_fn
	local fn="$REPLY"

	if [[ -v DEBUG ]]; then
		printf '\n\n%s\n\n' "function: $fn"
		printf '\n\n%s\n\n' "${COMP_WORDS[COMP_CWORD]} = ${fn##*::}"
	fi

	if declare -F "$fn" &>/dev/null; then
		if [ "${COMP_WORDS[COMP_CWORD]}" = "${fn##*::}" ]; then
			# for example for 'cmd aa',
			# when we are at 'cmd aa' (just after typing the a), we need
			# the completion for 'cmd' to run (not 'cmd aa'). That way, 'aa' won't be
			# overriden. This forces that.
			"${fn%::*}"
		else
			"$fn"
		fi
	else
		# for 'cmd aa bb cc',
		# this ensures that 'cmd a' works
		fn=${fn%::*}
		if declare -F "$fn" &>/dev/null ; then
			"$fn"
		fi
	fi
}

complete -F _dotmgr dotmgr
