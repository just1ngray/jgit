# jgit

Justin's git wrapper to help manage repositories with worktrees mapping onto branches.

## Install

Grab an appropriate pre-compiled binary from [releases](https://github.com/just1ngray/jgit/releases).

You can call it directly wherever it lives, and you're done! But it's easier if you 
add it somewhere to your path so you can execute it like `jgit` without needing the
fully resolved path to it (e.g., `~/Downloads/jgit_x86_64-unknown-linux-musl`).

```shell
$ echo $PATH
... /home/justin/.local/bin /usr/local/sbin /usr/local/bin /usr/sbin /usr/bin /sbin /bin /usr/games /usr/local/games /snap/bin

# choose an appropriate target
$ cp ~/Downloads/jgit_x86_64-unknown-linux-musl ~/.local/bin
```

If a compatible binary isn't available or you want the latest non-released version:

```
git clone https://github.com/just1ngray/jgit.git
cd jgit
cargo build --release
target/release/jgit --version
```

### Auto-complete

Once the binary is in-place, you can choose to add auto-completion for your shell.

```shell
jgit autocomplete --help
jgit autocomplete --shell fish generate
jgit autocomplete --shell fish install
```

### Uninstall

```shell
# if you installed autocompletions, uninstall them for each relevant shell
$ jgit autocomplete --shell xyz uninstall

# then, remove the binary itself
$ rm $(command -v jgit)
```

## Main usage commands

These commands are enough to get up and running with jgit. Firstly, you need to
clone a repository with jgit, and then you need to grab some branches. Once that's
done it's easy to hop into each branch and start developing.

Read more about [git-worktree](https://git-scm.com/docs/git-worktree), but in short
it allows you to checkout multiple branches in different directories efficiently.
Open each branch in your preferred editor and run git commands on it directly. Some
helpful commands are:

```shell
git worktree list
git worktree remove <worktree>
```

### clone

Use `jgit clone` command to clone a git repository as a jgit-compatible worktree repo.

```shell
jgit clone https://github.com/just1ngray/jgit.git
```

### branch

Use `jgit branch` inside any jgit repository folder to create a new worktree for a given
branch name. In jgit, branches and worktrees map onto each other directly and share the
same name. Raw git doesn't impose such restrictions and worktrees may serve any branch
not already checked out in another worktree.

```shell
# create a worktree for the master branch
$ jgit branch master

# create a worktree for a new branch called feat-123
$ jgit branch feat-123
# ... to avoid prompting you can specify the source branch
$ jgit branch feat-123 master
```

## Helper commands

### tree

Shows a tree of repositories and branches in a [tree](https://en.wikipedia.org/wiki/Tree_(command)) 
structure. This is helpful for finding branches/worktrees within a jgit repository, as
well as navigating your 'repos' folder wherever it's stored.

```shell
$ jgit tree

# or, ignoring branches
$ jgit tree -b
```

### clean

Cleans up a jgit repository by doing two things:

1. Deleting worktrees whose branches are no longer tracked remotely (e.g., merged)
2. Removing branches from your local refs which are not checked out by any worktree

By default you must confirm deletions but this can be bypassed with `-y` flag.

```shell
jgit clean
jgit clean -y
```
