#!/bin/bash

binpath="/usr/local/bin/jgit"

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

if [ ! -L $binpath ]; then
    echo "jgit symlink does not exist"
    exit 0
fi

sudo rm $binpath
echo "jgit symlink has been removed"
