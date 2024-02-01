#!/bin/bash

binpath="/usr/local/bin/jgit"
completionpath="/etc/bash_completion.d/jgit"

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

is_installed () {
    if [ ! -L $binpath ]; then
        return 1
    fi

    if [ ! -L $completionpath ]; then
        return 1
    fi

    return 0
}