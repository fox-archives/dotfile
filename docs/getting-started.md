# Getting Started

## Prerequisites

- Familiarity with Bash scripting
- Familiarity with Git
- **Pre-existing usage of a dotfile synchronizer**

The last one is _very important_. `dotmgr` is mostly a script manager and you need to already have a way to synchronize dotfiles, like [Chezmoi](https://github.com/twpayne/chezmoi), [dotdrop](https://github.com/deadc0de6/dotdrop), or [rcm](https://github.com/thoughtbot/rcm).

## Installation

There are two contexts in which dotmgr can be installed:

- When bootstrapping your dotfiles
- Initial installation (you're doing this right now)

We recommend installing them to the same location. I'll use `~/.dotmgr` since it's simple and the initialization scripts in the next step use it by default. You can always change it later

```sh
git clone 'https://github.com/hyperupcall/dotmgr' ~/.dotmgr
```

You're going to have to add `~/.dotmgr/src/bin` to the path manually, in `~/.bashrc` or whatever. Just make sure that the file is already managed by a dotfile synchronizer like [Chezmoi](https://github.com/twpayne/chezmoi) or your personal tool of choice

I personally symlink the `dotmgr` executable to a [common bin directory](https://github.com/hyperupcall/dots/blob/5066bd6b29586f90ed2dd2db550eeb22c2fc96e6/dotmgr/bootstrap.sh#L43) and [add _that_ to the PATH](https://github.com/hyperupcall/dots/blob/5066bd6b29586f90ed2dd2db550eeb22c2fc96e6/user/.config/shell/profile.sh#L27). Since `profile.sh` is handled (it creates a symlink from `~/.profile`) by my personal dotfile synchronizer, [dotfox](https://github.com/hyperupcall/dotfox), everything just works

## Initialization

You're almost ready to rock and roll!

Simply follow the insturctions. Once ran, you're ready for [Guide](docs/guide.md)
