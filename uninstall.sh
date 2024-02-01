#!/bin/bash

abs_proj_path=$(dirname "$(readlink -f "$0")")
source "$abs_proj_path/util.sh"

if ! is_installed; then
    echo "jgit is not installed"
    exit 0
fi

sudo rm "$completionpath"
sudo rm "$binpath"

echo "jgit has been uninstalled"
