#!/bin/bash

binpath="/usr/local/bin/jgit"

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
  exit 1
fi

if [ -e $binpath ]; then
    echo "jgit is already installed ($binpath)"
    exit 1
fi

jgit_path=$(dirname "$(readlink -f $0)")/jgit.sh
sudo ln -sf "$jgit_path" $binpath

echo "You can now run jgit as a program using 'jgit <command>'"
echo "Please do not delete this source directory, as it is required for jgit to function"
