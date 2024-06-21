#!/bin/bash

_jgit_completion() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    prev2="${COMP_WORDS[COMP_CWORD-2]}"

    if [[ "$prev" == "jgit" ]]; then
        opts="repo branch clean help -h --help"
        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
        return 0
    fi

    if [[ "$prev" == "branch" || "$prev2" == "branch" ]]; then
        opts=$(git branch -a | sed 's/remotes\/origin\///')
        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
        return 0
    fi
}

complete -F _jgit_completion jgit
