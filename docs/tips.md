# Tips

1. Do not create functions starting with `_`, `core`, or `term`

These are reserved by use by dotmgr itself and the "pacakges" it depends on. This is because your scripts are `source`'d and name conflicts must be prevented.

2. `DOTMGR_DIR` override

Set this environment variable to set the location of the dotmgr content directory. This is done in this very repository so you can use the example.

3. Use `core.*` functions

My [`bash-core`](https://github.com/hyperupcall/bash-core) [`bash-term`](https://github.com/hyperupcall/bash-term) libraries are vendored directly into this project, so use any functions from there to make script writing a bit easier. But, they are likely not the latest versions, so be careful when perusing respective docs.

4. Error: `Failed to find your dotmgr directory`

This means dotmgr was not able to deduce your dotmgr _content directory_. This is the directory that holds your actions, hooks, profiles, and other scripts.

By default, it will try to use `~/.dotfiles/dotmgr` and `~/.dots/dotmgr`. If neither exist, it will error. Avoid the error by writing a `.dotmgr_dir` file with the content being the full path to your dotmgr content directory.

For example, if you cloned this repository to `~/.dotmgr` and your dotmgr content directory is at `~/my-dotfiles/dotmgr`, then write the file `~/.dotmgr/.dotmgr_dir` with the content `~/my-dotfiles/dotmgr`.
