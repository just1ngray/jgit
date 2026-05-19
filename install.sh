#!/bin/bash

abs_proj_path=$(dirname "$(readlink -f "$0")")
source "$abs_proj_path/util.sh"

mkdir -p $(dirname "$install_path")
ln -sf "$abs_proj_path/jgit.sh" "$install_path"

bash_completions="${HOME:?HOME is unknown}/.local/share/bash_completion/completions/jgit"
mkdir -p $(dirname "$bash_completions")
ln -sf "$abs_proj_path/completions/bash.sh" "$bash_completions"

if command -v fish >/dev/null 2>&1; then
    fish_completions="${HOME:?HOME is unknown}/.config/fish/completions/jgit.fish"
    mkdir -p $(dirname "$fish_completions")
    ln -sf "$abs_proj_path/completions/jgit.fish" "$fish_completions"
fi

echo "You can now run jgit as a program using 'jgit <command>'"
echo "Do not delete this source directory, as it is required for jgit to function"
echo "Restart your shell to use jgit auto-complete features"
