# jgit

```
JGit - Justin's simple git repository and worktree manager.

    As developers we multi-task: a bugfix here, a PR review there, a couple of
    features being developed in parallel. Stashing changes is a nightmare, and
    sometimes your spahgetti code isn't quite ready to be committed.

    Worktrees are an excellent tool for solving this problem. They are similar
    to cloning the repository multiple times, but without the overhead since
    they share the same git object store. This means that you can have multiple
    branches checked out at the same time, and you can switch between them
    simply by changing your directory.

    In general, worktrees don't enforce many branch rules. With jgit we gently
    enforce that each worktree tracks a remote branch, and that the worktree
    checks out a branch whose name is the same as the path of the worktree,
    relative to the root of the jgit-cloned repository.

    Note: this utility does not replace git, and still requires a fundamental
          understanding of git and its basic commands.

Usage: /usr/local/bin/jgit {repo|branch|clean|help} [args]

    repo <name> <url>
        Create a worktree repository
    branch <name> [from]
        Create a new worktree for the given branch 'name'.
        The 'from' argument is never needed, and is intended for advanced use:
            If 'name' does not exist on remote, it will be created from the
            'from' branch if 'from' exists. If 'from' is not provided, you will
            be prompted to choose a 'from' branch (with the default being the
            repo's default branch).
    clean
        Delete local worktrees that do not have corresponding remote branches,
        and deletes branches which are not checked out by any worktree. This
        does not delete remote branches.
    help
        Prints this message

Typical usage:

    1. Clone jgit repository to a directory
        $ git clone https://github.com/just1ngray/jgit.git
    2. Install jgit program to your system (optional - can be called directly
        from the source directory)
        $ cd jgit
        $ sudo ./install.sh
    3. Create a new worktree repository (perhaps in the same directory as the
        jgit source)
        $ jgit repo hello-world https://github.com/leachim6/hello-world.git
    4. Create a new worktree for a given branch
        $ cd hello-world
        $ jgit branch main
    5. Clean up old worktrees whose branch no longer exists on remote (like
        after you've merged a PR and deleted the remote branch)
        $ jgit clean

To uninstall jgit:

    1. Run the uninstall script
        $ sudo ./uninstall.sh
    2. Remove the jgit directory you are currently inside of
        $ rm -rf ../jgit
```
