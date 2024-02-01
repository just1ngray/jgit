# jgit

```
JGit - Justin's simple git repository and worktree manager

Usage: jgit {repo|branch|clean} [args]

    repo <name> <url>   Create a worktree repository
    branch <name>       Create a new worktree for the given branch
    clean               Delete local worktrees that do not have corresponding
                        remote branches

Typical usage:
    1. Clone jgit repository to a directory
        $ git clone https://github.com/just1ngray/jgit.git
    2. Install jgit program to your system
        $ cd jgit
        $ sudo ./install.sh
    3. Create a new worktree repository (perhaps in the same directory as the
        jgit source)
        $ jgit repo hello-world https://github.com/leachim6/hello-world.git
    4. Create a new worktree for the given branch
        $ cd hello-world
        $ jgit branch main
    5. Clean up old worktrees whose branch no longer exists on remote
        $ jgit clean
```
