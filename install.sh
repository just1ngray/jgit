#!/bin/bash

abs_proj_path=$(dirname "$(readlink -f "$0")")
source "$abs_proj_path/util.sh"

if is_installed; then
    echo "jgit is already installed"
    exit 0
fi

jgit_path="$abs_proj_path/jgit.sh"
completion_path="$abs_proj_path/bash_completion.sh"

sudo ln -sf "$jgit_path" "$binpath"
sudo ln -sf "$completion_path" "$completionpath"

echo "You can now run jgit as a program using 'jgit <command>'"
echo "Do not delete this source directory, as it is required for jgit to function"
echo "Restart your shell to use jgit auto-complete features"
