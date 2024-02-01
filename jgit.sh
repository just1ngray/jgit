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

    if ! git branch --set-upstream-to=origin/"$name"; then
        echo "Branch does not exist on remote repository"
        echo "On your first push you will have to execute:"
        echo "      git push --set-upstream origin $name"
    fi
}

# Delete local worktrees that do not have corresponding remote branches
clean () {
    git fetch --prune

    remote_branches=$(git branch --remotes | grep -Po "origin/\K.+")
    local_worktree_branches=$(git worktree list | grep -Po "\[.+\]$" | tr -d '[]')

    keep=""
    remove=""

    for branch in $local_worktree_branches; do
        if ! echo "$remote_branches" | grep -q "^$branch$"; then
            remove+="$branch "
        else
            keep+="$branch "
        fi
    done

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
    *)
        echo "Usage: $0 {repo|branch|clean} [args]"
        echo "Unknown command: '$1'"
        exit 1
        ;;
esac
