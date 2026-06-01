# Clear out any existing completions for jgit to prevent overlaps
complete -c jgit -e

# Helper to check if no subcommand has been provided yet
function __fish_jgit_needs_command
    set -l cmd (commandline -opc)
    if test (count $cmd) -eq 1
        return 0
    end
    return 1
end

# --- Primary Commands ---
complete -c jgit -f -n '__fish_jgit_needs_command' -a repo -d 'Create a worktree repository'
complete -c jgit -f -n '__fish_jgit_needs_command' -a branch -d 'Create a new worktree for a branch'
complete -c jgit -f -n '__fish_jgit_needs_command' -a clean -d 'Delete local worktrees missing remote branches'
complete -c jgit -f -n '__fish_jgit_needs_command' -a tree -d 'Print a tree of all jgit worktree repos'
complete -c jgit -f -n '__fish_jgit_needs_command' -a help -d 'Print help message'

# --- Context-Aware Subcommand Arguments ---

# branch/checkout: complete remote branches dynamically if inside a jgit repo
function __fish_jgit_remote_branches
    if test -e .bare
        git ls-remote --heads origin 2>/dev/null | awk -F'refs/heads/' '{print $2}'
    end
end
complete -c jgit -f -n '__fish_seen_subcommand_from branch checkout' -a '(__fish_jgit_remote_branches)' -d 'Remote branch'

# clean/prune/remove: auto-complete the 'yy' or '-y' flags
complete -c jgit -f -n '__fish_seen_subcommand_from clean prune remove' -a 'yy -y' -d 'Proceed without prompting'

# repo/clone: allow standard file and directory path completion
complete -c jgit -F -n '__fish_seen_subcommand_from repo clone'
