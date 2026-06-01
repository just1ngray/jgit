#!/bin/bash
# Bash completion script for jgit

_jgit() {
    local cur prev cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # Primary commands and their aliases
    cmds="repo clone branch checkout clean prune remove tree help -h --help"

    # Complete the primary command
    if [[ ${COMP_CWORD} -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "${cmds}" -- "${cur}") )
        return 0
    fi

    # Context-aware completion based on the subcommand
    local subcommand="${COMP_WORDS[1]}"

    case "${subcommand}" in
        branch|checkout)
            # If we are in a jgit top-level repo, dynamically complete remote branch names
            if [[ -e .bare ]]; then
                local branches=$(git ls-remote --heads origin 2>/dev/null | awk -F'refs/heads/' '{print $2}')
                COMPREPLY=( $(compgen -W "${branches}" -- "${cur}") )
            fi
            return 0
            ;;
        clean|prune|remove)
            # Auto-complete the bypass prompt flags
            if [[ ${COMP_CWORD} -eq 2 ]]; then
                COMPREPLY=( $(compgen -W "yy -y" -- "${cur}") )
            fi
            return 0
            ;;
        repo|clone)
            # Standard bash completion will fallback to directories for the path argument
            # due to the '-o default' flag on the complete command below
            return 0
            ;;
    esac
}

# Register the completion function with directory fallback
complete -o default -F _jgit jgit
