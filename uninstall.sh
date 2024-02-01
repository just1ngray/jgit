#!/bin/bash

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

if [ ! -L /usr/local/bin/jgit ]; then
    echo "jgit symlink does not exist"
    exit 0
fi

sudo rm /usr/local/bin/jgit
echo "jgit symlink has been removed"
