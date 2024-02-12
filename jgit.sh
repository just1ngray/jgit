#!/bin/bash

set -e

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

    # basic validation
    if [ -z "$name" ]; then
        echo "Usage: branch <name>"
        exit 1
    fi
    if [ ! -e .bare ]; then
        echo "This command must be executed from git repository"
        exit 1
    fi
    if [ -e "$name" ]; then
        echo "Branch '$name' already exists as a local worktree directory"
        exit 1
    fi

    # sync with remote
    git fetch

    # check if the selected branch exists; if not, then try creating it from another branch
    if ! git ls-remote --exit-code origin "$name" > /dev/null; then
        echo "Branch '$name' does not exist on remote"

        local from_branch=$2
        if [ -z "$from_branch" ]; then
            default_branch=$(git symbolic-ref HEAD | sed "s/refs\/heads\///")
            read -p "Create branch '$name' from which branch? ($default_branch): " from_branch
            if [ -z "$from_branch" ]; then
                from_branch=$default_branch
            fi
        fi

        if ! git ls-remote --exit-code origin "$from_branch" > /dev/null; then
            echo "Err: Branch '$from_branch' does not exist on remote"
            exit 1
        fi

        echo "Creating branch '$name' from '$from_branch'"
        git fetch origin "$from_branch":"$from_branch"
        git push origin "$from_branch":"$name"
    fi

    # create the worktree and set it to track exactly the remote branch
    git worktree add "$name"
    cd "$name"
    git checkout "$name"
    git branch --set-upstream-to=origin/"$name"
    git reset --hard origin/"$name"
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
    echo -e "$remove" | sed 's/ /\n    /g'

    echo -en "Proceed? y/(n): "
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

Usage: $0 {repo|branch|clean} [args]

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
