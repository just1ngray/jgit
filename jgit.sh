#!/bin/bash

# Create a worktree repository
repo () {
    local name=$1
    local url=$2

    if [ -z "$name" ] || [ -z "$url" ]; then
        echo "Usage: repo <name> <url>"
        exit 1
    fi

    if [ -d "$name" ]; then
        echo "Repository '$name' already exists"
        exit 1
    fi

    mkdir -p "$name"
    cd "$name"
    git clone --bare "$url" .bare
    echo "gitdir: .bare" > .git
    git config remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*"
}

# Create a new worktree for the given branch
branch () {
    local name=$1

    if [ -z "$name" ]; then
        echo "Usage: branch <name>"
        exit 1
    fi

    if [ ! -e .bare ]; then
        echo "This command must be executed from git repository"
        exit 1
    fi

    if [ -e "$name" ]; then
        echo "Branch '$name' already exists"
        exit 1
    fi

    git fetch
    git worktree add "$name"
    cd "$name"
    git checkout "$name"
    git branch --set-upstream-to=origin/"$name"
    git pull
}

# Delete local worktrees that do not have corresponding remote branches
clean () {
    if [ ! -e .bare ]; then
        echo "This command must be executed from your top-level worktree repository"
        exit 1
    fi

    git fetch --prune

    remote_branches=$(git branch --remotes | grep -Po "origin/\K.+")
    local_worktree_branches=$(git worktree list | grep -Po "\[.+\]$" | tr -d '[]')

    remove=""
    for branch in $local_worktree_branches; do
        if ! echo "$remote_branches" | grep -q "^$branch$"; then
            remove+="$branch "
        fi
    done

    if [ "$remove" = "" ]; then
        echo "Nothing to delete. Every worktree branch exists on remote"
        exit 0
    fi

    echo -e "\nWORKTREES TO DELETE"
    echo -n "    "
    echo -e "$remove" | sed 's/ /\n    /'

    echo -en "\nProceed? y/(n): "
    read proceed

    if [ "$proceed" != "y" ]; then
        echo "Exiting"
        exit 0
    fi

    for branch in $remove; do
        echo "Removing worktree $branch"
        git worktree remove "$branch" # do not -f (force)
    done

    echo -e "\nREMAINING WORKTREES:"
    git worktree list | sed 's/^/    /'
}


# Prints help and usage message
help () {
    echo """
JGit - Justin's simple git repository and worktree manager

Usage: $0 {repo|branch|clean} [args]

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

To uninstall jgit:
    1. Run the uninstall script
        $ sudo ./uninstall.sh
    2. Remove the jgit directory you are currently inside of
        $ rm -rf ../jgit
    """
}


case "$1" in
    repo)
        shift
        repo "$@"
        ;;
    branch)
        shift
        branch "$@"
        ;;
    clean)
        clean
        ;;
    help|-h|--help)
        help
        ;;
    *)
        help
        echo -e "\nUnknown command: '$1'"
        exit 1
        ;;
esac
