# Tips

## Writing Scripts

1. Do not create functions starting with `_`

These are reserved by use by dotmgr itself (your scripts are `source`'d and name conflicts must be prevented)

## Troubleshooting

1. `Failed to find your dotmgr directory`

This means dotmgr was not able to deduce your dotmgr _content directory_. This is the directory that holds your actions, hooks, profiles, and other scripts.

By default, it will try to use `~/.dotfiles/dotmgr` and `~/.dots/dotmgr`. If neither exist, it will error. Avoid the error by writing a `.dotmgr_dir` file with the content being the full path to your dotmgr content directory.

For example, if you cloned this repository to `~/.dotmgr` and your dotmgr content directory is at `~/my-dotfiles/dotmgr`, then write the file `~/.dotmgr/dotmgr_dir` with the content `~/my-dotfiles/dotmgr`
