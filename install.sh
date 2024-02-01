#!/bin/bash

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
  exit 1
fi

jgit_path=$(dirname "$(readlink -f $0)")/jgit.sh
sudo ln -sf "$jgit_path" /usr/local/bin/jgit

echo "You can now run jgit as a program using 'jgit <command>'"
echo "Please do not delete this source directory, as it is required for jgit to function"
