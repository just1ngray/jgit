# jgit

```
JGit - Justin's simple git repository and worktree manager.

    Git worktrees offer a powerful capability: allowing developers to
    simultaneously checkout multiple branches from a single repository by
    isolating them into distinct directories on disk. Unlike traditional
    cloning, worktrees efficiently share the same object database, reducing
    duplication and conserving disk/network space.

    This feature eliminates the need to stash or commit changes when switching
    between branches, streamlining development workflows. However, leveraging
    worktrees can be daunting for newcomers due to their complexity.

    Enter jgit, a solution designed to simplify the management of git
    worktrees. While jgit doesn't support all git worktree operations, it
    provides intuitive commands and workflows to facilitate common tasks,
    enabling smoother branch management and increased productivity in
    git-based projects.

Usage: /usr/local/bin/jgit {repo|branch|clean} [args]

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
        Delete local worktrees that do not have corresponding remote branches

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
