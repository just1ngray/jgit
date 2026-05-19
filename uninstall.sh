#!/bin/bash

abs_proj_path=$(dirname "$(readlink -f "$0")")
source "$abs_proj_path/util.sh"

rm "$install_path"
rm "${HOME:?HOME is unknown}/.config/fish/completions/jgit.fish"
rm "${HOME:?HOME is unknown}/.local/share/bash_completion/completions/jgit"

echo "jgit has been uninstalled"
