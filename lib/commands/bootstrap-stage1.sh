# shellcheck shell=bash

# Assumptions:
# sudo, git, nvim installed
# hyperupcall/dots cloned
# dotmgr in PATH

subcmd() {
	if [ -z "$XDG_CONFIG_HOME" ]; then
		util.die '$XDG_CONFIG_HOME is empty. Did you source profile-pre-bootstrap.sh?'
	fi

	if [ -z "$XDG_DATA_HOME" ]; then
		util.die '$XDG_DATA_HOME is empty. Did you source profile-pre-bootstrap.sh?'
	fi

	# Ensure prerequisites
	mkdir -p ~/.bootstrap/{bin,nim-all,old-homedots} "$XDG_CONFIG_HOME"

	if ! util.is_cmd jq; then
		printf '%s\n' 'Installing jq'

		if util.is_cmd pacman; then
			ensure sudo pacman -S --noconfirm jq &>/dev/null
		elif command -v apt-get &>/dev/null; then
			ensure sudo apt-get -y install jq &>/dev/null
		elif util.is_cmd dnf; then
			ensure sudo dnf -y install jq &>/dev/null
		elif util.is_cmd zypper; then
			ensure sudo zypper -y install jq &>/dev/null
		elif util.is_cmd eopkg; then
			ensure sudo eopkg -y install jq &>/dev/null
		fi

		if ! util.is_cmd jq; then
			die 'Automatic installation of jq failed'
		fi
	fi

	if ! util.is_cmd curl; then
		util.log_info 'Installing curl'

		if util.is_cmd pacman; then
			util.ensure sudo pacman -S --noconfirm curl &>/dev/null
		elif util.is_cmd apt-get &>/dev/null; then
			util.ensure sudo apt-get -y install curl &>/dev/null
		elif util.is_cmd dnf; then
			util.ensure sudo dnf -y install curl &>/dev/null
		elif util.is_cmd zypper; then
			util.ensure sudo zypper -y install curl &>/dev/null
		elif util.is_cmd eopkg; then
			util.ensure sudo eopkg -y install curl &>/dev/null
		fi

		if ! util.is_cmd curl; then
			util.die 'Automatic installation of curl failed'
		fi
	fi

	# Remove distribution specific dotfiles, including
	for file in ~/.bash_login ~/.bash_logout ~/.bash_profile ~/.bashrc ~/.profile; do
		if [ -f "$file" ]; then
			util.ensure mv "$file" ~/.bootstrap/old-homedots
		fi
	done

	# Download Nim (in case dotfox doesn't work, it may need to be recompiled)
	if [ ! -d ~/.bootstrap/nim-all/nim ]; then
		util.log_info 'Downloading Nim'
		util.ensure curl -LSso ~/.bootstrap/nim-all/nim-1.4.8-linux_x64.tar.xz https://nim-lang.org/download/nim-1.4.8-linux_x64.tar.xz
		util.ensure rm -rf ~/.bootstrap/nim-all/nim-1.4.8
		util.ensure cd ~/.bootstrap/nim-all
		util.ensure tar xf nim-1.4.8-linux_x64.tar.xz
		util.ensure cd
		util.ensure ln -sTf ~/.bootstrap/nim-all/nim-1.4.8 ~/.bootstrap/nim-all/nim
	fi

	# Clone dotfox
	if [ ! -d ~/.bootstrap/dotfox ]; then
		util.log_info 'Cloning github.com/hyperupcall/dotfox'
		util.ensure git clone --quiet https://github.com/hyperupcall/dotfox ~/.bootstrap/dotfox
	fi

	# Download Dotfox
	if [ ! -f ~/.bootstrap/bin/dotfox ]; then
		util.log_info 'Downloading Dotfox'
		if ! dotfox_download_url="$(
			curl -LfSs https://api.github.com/repos/hyperupcall/dotfox/releases/latest \
				| jq -r '.assets[0].browser_download_url'
		)"; then
			util.die "Could not fetch the dotfox download URL"
		fi
		util.ensure curl -LsSo ~/.bootstrap/bin/dotfox "$dotfox_download_url"
		util.ensure chmod +x ~/.bootstrap/bin/dotfox
	fi

	# Download Basalt
	if [ ! -d "$XDG_DATA_HOME/basalt/source" ]; then
		util.log_info 'Downloading Basalt'
		util.ensure git clone --quiet https://github.com/hyperupcall/basalt "$XDG_DATA_HOME/basalt/source"
	fi

	# Install Homebrew
	if [ "$OSTYPE" = Darwin ]; then
		util.ensure curl -LsSo ~/.bootstrap/brew-install.sh https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh
		util.ensure chmod +x ~/.bootstrap/brew-install.sh
		~/.bootstrap/brew-install.sh
	fi

	# Download Basalt
	if [ ! -d "$XDG_DATA_HOME/basalt/source" ]; then
		util.log_info 'Downloading Basalt'
		util.ensure git clone --quiet https://github.com/hyperupcall/basalt "$XDG_DATA_HOME/basalt/source"
		util.ensure git -C "$XDG_DATA_HOME/basalt/source" submodule init
		util.ensure git -C "$XDG_DATA_HOME/basalt/source" submodule update
	fi

	if basalt_output="$("$XDG_DATA_HOME/basalt/source/pkg/bin/basalt" global init sh)"; then
		eval "$basalt_output"
	else
		util.die "Could not run 'basalt global init sh'"
	fi

	# ----------------------------------------------------------------------------------------------------------

	if util.is_cmd pacman; then
		util.log_info 'Updating, upgrading, and installing packages'
		sudo pacman -Syyu --noconfirm

		sudo pacman -Syu --noconfirm base-devel
		sudo pacman -Syu --noconfirm lvm2
		# sudo pacman -Syu --noconfirm pkg-config openssl
		# sudo pacman -Syu --noconfirm browserpass-chrome

		sudo pacman -Syu --noconfirm rsync xclip
	elif util.is_cmd apt-get; then
		util.log_info 'Updating, upgrading, and installing packages'
		sudo apt-get -y update
		sudo apt-get -y upgrade

		sudo apt-get -y install build-essential
		sudo apt-get -y install lvm2
		sudo apt-get -y install pkg-config libssl-dev # for starship
		sudo apt-get -y install webext-browserpass

		sudo apt-get -y install rsync xclip
	elif util.is_cmd dnf; then
		util.log_info 'Updating, upgrading, and installing packages'
		sudo dnf -y update
		sudo dnf -y upgrade

		sudo dnf -y install lvm2
		sudo dnf -y install pkg-config openssl-devel # for starship
		# sudo dnf -y install browserpass

		sudo dnf -y install rsync xclip
	fi

	dotmgr module rust
	if ! util.is_cmd starship; then
		util.log_info 'Installing starship'
		cargo install starship
	fi

	util.log_info 'Installing Basalt packages globally'
	basalt global add hyperupcall/choose hyperupcall/autoenv hyperupcall/dotshellextract hyperupcall/dotshellgen
	basalt global add cykerway/complete-alias rcaloras/bash-preexec
	basalt global add hedning/nix-bash-completions dsifford/yarn-completion

	# TODO
	# sudo lvchange -ay /dev/fox

	cat > ~/.bootstrap/stage2.sh <<-"EOF"
		. ~/.bootstrap/stage1.sh
		export PATH="$HOME/.bootstrap/dotfox:$HOME/.bootstrap/bin:$XDG_DATA_HOME/basalt/source/pkg/bin:$HOME/.bootstrap/nim-all/nim/bin:$PATH"

		if basalt_output="$("$XDG_DATA_HOME/basalt/source/pkg/bin/basalt" global init sh)"; then
			eval "$basalt_output"
		else
			printf '%s\n' "Could not run 'basalt global init sh'"
		fi
	EOF

	cat <<-"EOF"
	---
	. ~/.bootstrap/stage2.sh
	dotfox --config-dir="$HOME/.dots/user/.config/dotfox" --deployment=all.sh deploy
	dotmgr maintain
	. ~/.bashrc
	dotmgr bootstrap-stage2
	---
	EOF
}
