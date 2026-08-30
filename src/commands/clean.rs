use std::path::PathBuf;

use colored::Colorize;

use crate::commands::{JGitCli, RunCommand};

#[derive(Debug, clap::Args)]
pub struct CleanCommand {
    /// Auto-confirm clean operations without any verification or approval step
    #[arg(short = 'y')]
    autoconfirm: bool,
}

impl RunCommand for CleanCommand {
    fn run(&self, root: &JGitCli) {
        let cwd = std::env::current_dir().expect("Could not determine current directory");
        if !cwd.join(".bare").exists() {
            eprintln!(
                "{}",
                "This command must be executed from your top-level worktree repository".red()
            );
            std::process::exit(1);
        }

        let git = crate::git::Git::new(root.git.clone(), cwd.clone());
        git.run(["fetch", "--prune"]).assert_success();

        let prompt = !self.autoconfirm;
        if !prompt {
            eprintln!(
                "{}",
                "!! Proceeding to clean worktrees and branches without prompting for confirmation"
                    .yellow()
            );
        }

        self.clean_worktrees(root, &cwd, prompt);
        self.clean_branches(root, &cwd, prompt);
    }
}

impl CleanCommand {
    fn clean_worktrees(&self, root: &JGitCli, cwd: &PathBuf, prompt: bool) {
        let git = crate::git::Git::new(root.git.clone(), cwd.clone());
        let remote_branches = Self::get_remote_branches(&git);
        let worktree_branches = Self::get_worktree_branches(&git);

        let remove: Vec<String> = worktree_branches
            .into_iter()
            .filter(|b| !remote_branches.contains(b))
            .collect();

        if remove.is_empty() {
            eprintln!(
                "{}",
                "No worktrees to delete. Every worktree branch exists on remote".green()
            );
            return;
        }

        eprintln!("\n{}", "WORKTREES TO DELETE".yellow().bold());
        for branch in &remove {
            eprintln!("    {}", branch.yellow());
        }

        if prompt && !Self::confirm("Proceed? y/(n): ") {
            eprintln!("{}", "Cancelling worktree removal".red());
            return;
        }

        let mut must_force = vec![];
        for branch in &remove {
            let branch_path = cwd.join(branch);
            let branch_git = crate::git::Git::new(root.git.clone(), branch_path.clone());
            let status = branch_git.run(["status", "--porcelain"]);
            if !status.stdout.trim().is_empty() {
                eprintln!(
                    "{}",
                    format!("  -> Skipping: '{branch}' contains untracked or modified files.")
                        .yellow()
                );
                must_force.push(branch.clone());
                continue;
            }

            eprintln!("{}", format!("Removing worktree {branch}").cyan());
            // remove with --force. we know there are no untracked files, so --force
            // removal to resolve any issues with git-submodules in the worktree
            git.run(["worktree", "remove", branch.as_str(), "--force"])
                .assert_success();

            // remove (now) empty directories since the worktree was removed
            // ... this happens when the branch name contains some '/' and
            //     the worktree is created in a subdirectory
            if let Some(parent) = branch_path.parent() {
                let mut dir = parent.to_path_buf();
                while dir != *cwd {
                    if std::fs::remove_dir(&dir).is_err() {
                        break;
                    }
                    eprintln!(
                        "{}",
                        format!("Deleted empty directory {}", dir.display()).cyan()
                    );
                    match dir.parent() {
                        Some(p) => dir = p.to_path_buf(),
                        None => break,
                    }
                }
            }
        }

        eprintln!("\n{}", "---------------------".dimmed());

        if !must_force.is_empty() {
            eprintln!(
                "\n{}",
                "Untracked files. Inspect, and possibly remove with --force:".yellow()
            );
            for branch in &must_force {
                eprintln!(
                    "{}",
                    format!("    git worktree remove '{branch}' --force").yellow()
                );
            }
        }

        eprintln!("\n{}", "REMAINING WORKTREES:".green().bold());
        for line in git.run(["worktree", "list"]).stdout.lines() {
            eprintln!("    {line}");
        }
    }

    fn clean_branches(&self, root: &JGitCli, cwd: &PathBuf, prompt: bool) {
        let git = crate::git::Git::new(root.git.clone(), cwd.clone());
        let local_branches = Self::get_local_branches(&git);
        let worktree_branches = Self::get_worktree_branches(&git);

        let remove: Vec<String> = local_branches
            .into_iter()
            .filter(|b| !worktree_branches.contains(b))
            .collect();

        if remove.is_empty() {
            eprintln!(
                "{}",
                "No branches to delete. Every branch is checked out on a worktree".green()
            );
            return;
        }

        eprintln!("\n{}", "BRANCHES TO DELETE".yellow().bold());
        for branch in &remove {
            eprintln!("    {}", branch.yellow());
        }

        if prompt && !Self::confirm("Proceed? y/(n): ") {
            eprintln!("{}", "Cancelling branch removal".red());
            return;
        }

        for branch in &remove {
            git.run(["branch", "-D", branch.as_str()]).assert_success();
        }

        eprintln!("\n{}", "REMAINING BRANCHES:".green().bold());
        for line in git.run(["branch"]).stdout.lines() {
            eprintln!("    {line}");
        }
    }

    fn get_local_branches(git: &crate::git::Git) -> Vec<String> {
        git.run(["branch", "--format=%(refname:short)"])
            .stdout
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn get_worktree_branches(git: &crate::git::Git) -> Vec<String> {
        git.run(["worktree", "list"])
            .stdout
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.ends_with(']') {
                    line.rfind('[')
                        .map(|start| line[start + 1..line.len() - 1].to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_remote_branches(git: &crate::git::Git) -> Vec<String> {
        git.run(["branch", "--remotes"])
            .stdout
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.contains("->") {
                    return None;
                }
                line.strip_prefix("origin/").map(|s| s.to_string())
            })
            .collect()
    }

    fn confirm(message: &str) -> bool {
        use std::io::Write;
        eprint!("{}", message.cyan());
        std::io::stderr().flush().ok();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");

        input.trim().to_lowercase().starts_with('y')
    }
}
