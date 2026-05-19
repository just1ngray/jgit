#!/bin/bash

if [ "$EUID" -eq 0 ]; then
    echo "Do not run as root"
    exit 1
fi

install_path="${HOME:?HOME is unknown}/.local/bin/jgit"

is_installed () {
    if [ ! -L "$install_path" ]; then
        return 1
    fi
    return 0
}
