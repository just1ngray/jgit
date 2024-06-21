#!/bin/bash

set -e

exec 3>&1 # Save the original stdout (file descriptor 1)
trap 'exec 3>&-' EXIT # Ensure file descriptor 3 is closed upon script exit
exec 1>&2 # Redirect all stdout to stderr

# One run, stdout redirection is undone
stdout() {
    exec 1>&3
    echo "$@"
}

# Create a worktree repository
repo () {
    local url=$1
    local path=${2:-$(grep -Po "[^/]+\.git$" <<< "$url" | sed "s/.git$//")}

    if [ -z "$path" ] || [ -z "$url" ]; then
        help
        echo "Usage: repo <url> [path]"
        exit 1
    fi

    if [ -d "$path" ]; then
        echo "Path '$path' already exists"
        exit 1
    fi

    echo "Creating folder to hold jgit worktree repository at: $path"
    mkdir -p "$path"
    cd "$path"
    git clone --bare "$url" .bare
    echo "gitdir: .bare" > .git
    git config remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*"

    stdout "$path"
}

# Create a new worktree for the given branch
branch () {
    local name=$1

    # basic validation
    if [ -z "$name" ]; then
        help
        echo "Usage: branch <name> [from]"
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
    git worktree add -B "$name" "$name"
    cd "$name"
    git checkout "$name"
    git branch --set-upstream-to=origin/"$name"
    git reset --hard origin/"$name"
    git pull

    stdout "$name"
}

_clean_worktrees() {
    remote_branches=$(git branch --remotes | grep -Po "origin/\K.+")
    local_worktree_branches=$(git worktree list | grep -Po "\[.+\]$" | tr -d '[]')

    remove=""
    for branch in $local_worktree_branches; do
        if ! echo "$remote_branches" | grep -q "^$branch$"; then
            remove+="$branch "
        fi
    done

    if [ "$remove" = "" ]; then
        echo "No worktrees to delete. Every worktree branch exists on remote"
        return
    fi

    echo -e "\nWORKTREES TO DELETE"
    echo -n "    "
    echo -e "$remove" | sed 's/ /\n    /g'

    echo -en "Proceed? y/(n): "
    read proceed

    if ! [[ "$proceed" =~ [Yy].* ]]; then
        echo "Cancelling worktree removal"
        return
    fi

    for branch in $remove; do
        echo "Removing worktree $branch"
        git worktree remove "$branch" # do not -f (force)
    done

    echo -e "\nREMAINING WORKTREES:"
    git worktree list | sed 's/^/    /'
}

# delete local branches that are not checked out to a worktree
_clean_branches() {
    local_branches=$(git branch --format='%(refname:short)')
    local_worktree_branches=$(git worktree list | grep -Po "\[.+\]$" | tr -d '[]')

    remove=""
    for branch in $local_branches; do
        if ! echo "$local_worktree_branches" | grep -q "^$branch$"; then
            remove+="$branch "
        fi
    done

    if [ "$remove" = "" ]; then
        echo "No branches to delete. Every branch is checked out on a worktree"
        return
    fi

    echo -e "\nBRANCHES TO DELETE"
    echo -n "    "
    echo -e "$remove" | sed 's/ /\n    /g'

    echo -en "Proceed? y/(n): "
    read proceed

    if ! [[ "$proceed" =~ [Yy].* ]]; then
        echo "Cancelling branch removal"
        return
    fi

    for branch in $remove; do
        git branch -D "$branch"
    done

    echo -e "\nREMAINING BRANCHES:"
    git branch
}

# Delete local worktrees that do not have corresponding remote branches, and
# deletes branches which are not attached to any worktree
clean () {
    if [ ! -e .bare ]; then
        echo "This command must be executed from your top-level worktree repository"
        exit 1
    fi

    git fetch --prune

    _clean_worktrees
    _clean_branches
}


# Prints help and usage message
help () {
    stdout """
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

Usage: $0 {repo|branch|clean|help} [args]

    repo <url> [path]
        Create a jgit-supported worktree repository from a standard clone URL,
        optionally saved to a particular path on disk.
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
        Prints this message.

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

Tip:

    If you want to open a new 'repo' or 'branch' in your editor, you can
    capture the output from the command. E.g.,
        $ code \$(jgit repo https://github.com/just1ngray/jgit.git)

To uninstall jgit:

    1. Run the uninstall script
        $ sudo ./uninstall.sh
    2. Remove the jgit directory you are currently inside of
        $ rm -rf ../jgit
    """
}


case "$1" in
    repo*|clone)
        shift
        repo "$@"
        ;;
    branch|checkout)
        shift
        branch "$@"
        ;;
    clean*|prune|remove)
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
