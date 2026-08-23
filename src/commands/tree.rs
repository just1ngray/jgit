use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use crate::commands::{Cli, RunCommand};

#[derive(Debug, clap::Args)]
pub struct TreeCommand {
    /// Only show jgit repositories, hiding their worktrees
    #[arg(short = 'b')]
    hide_branches: bool,
}

impl RunCommand for TreeCommand {
    fn run(&self, root: &Cli) {
        let show_worktrees = !self.hide_branches;
        let cwd = match std::env::current_dir() {
            Ok(cwd) => cwd,
            Err(error) => {
                eprintln!("Could not determine current directory: {error}");
                std::process::exit(1);
            }
        };

        let repos = Self::find_repos(&cwd);
        if repos.is_empty() {
            eprintln!("No jgit repositories found in {}", cwd.display());
            return;
        }

        let mut tree_input = String::new();
        let mut repo_count = 0;
        let mut worktree_count = 0;

        for repo in &repos {
            repo_count += 1;

            let repo_display = if repo == Path::new(".") {
                ".".to_string()
            } else {
                repo.display().to_string()
            };
            tree_input.push_str(&repo_display);
            tree_input.push('\n');

            if !show_worktrees {
                continue;
            }

            let prefix = if repo_display == "." {
                String::new()
            } else {
                format!("{repo_display}/")
            };

            let repo_path = cwd.join(repo);
            let repo_abs = std::fs::canonicalize(&repo_path).unwrap_or(repo_path.clone());
            let git = crate::git::Git::new(root.git.clone(), repo_path);
            let worktrees = Self::get_worktrees(&git);

            if worktrees.is_empty() {
                tree_input.push_str(&format!("{prefix}(no worktrees)\n"));
            } else {
                for wt in &worktrees {
                    worktree_count += 1;
                    let rel_wt = wt.strip_prefix(&repo_abs).unwrap_or(wt);
                    let flat_wt = rel_wt.display().to_string().replace('/', "∕");
                    tree_input.push_str(&format!("{prefix}{flat_wt}\n"));
                }
            }
        }

        Self::print_tree(&tree_input);

        eprintln!();
        if show_worktrees {
            eprintln!("{repo_count} jgit repositories, {worktree_count} worktrees");
        } else {
            eprintln!("{repo_count} jgit repositories");
        }
    }
}

impl TreeCommand {
    /// Recursively find directories containing a '.bare' folder, returning
    /// their path relative to `root` (or '.' for `root` itself).
    fn find_repos(root: &Path) -> Vec<PathBuf> {
        let mut repos = vec![];
        let mut stack = vec![root.to_path_buf()];

        while let Some(dir) = stack.pop() {
            let entries = match std::fs::read_dir(&dir) {
                Ok(entries) => entries,
                Err(_) => continue,
            };

            for entry in entries.flatten() {
                let file_type = match entry.file_type() {
                    Ok(file_type) => file_type,
                    Err(_) => continue,
                };
                if !file_type.is_dir() {
                    continue;
                }

                let path = entry.path();
                if path.file_name().is_some_and(|name| name == ".bare") {
                    let rel = path
                        .parent()
                        .and_then(|p| p.strip_prefix(root).ok())
                        .filter(|p| !p.as_os_str().is_empty())
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| PathBuf::from("."));
                    repos.push(rel);
                } else {
                    stack.push(path);
                }
            }
        }

        repos.sort();
        repos
    }

    /// Get the absolute paths of all worktrees for a repo, excluding its
    /// main '.bare' entry.
    fn get_worktrees(git: &crate::git::Git) -> Vec<PathBuf> {
        git.run(["worktree", "list"])
            .stdout
            .lines()
            .filter_map(|line| {
                let path_str = line.split_whitespace().next()?;
                if path_str.ends_with(".bare") {
                    return None;
                }
                Some(PathBuf::from(path_str))
            })
            .collect()
    }

    fn print_tree(tree_input: &str) {
        let mut child = match std::process::Command::new("tree")
            .args(["--noreport", "--fromfile", "."])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                eprintln!("Could not execute 'tree': {error}");
                std::process::exit(1);
            }
        };

        let mut tree_stdout = match child.stdout.take() {
            Some(stdout) => stdout,
            None => {
                eprintln!("Could not capture 'tree' output");
                std::process::exit(1);
            }
        };
        let output_thread =
            std::thread::spawn(move || std::io::copy(&mut tree_stdout, &mut std::io::stderr()));

        let mut stdin = match child.stdin.take() {
            Some(stdin) => stdin,
            None => {
                eprintln!("Could not write to 'tree' stdin");
                std::process::exit(1);
            }
        };
        if let Err(error) = stdin.write_all(tree_input.as_bytes()) {
            eprintln!("Could not write to 'tree' stdin: {error}");
            std::process::exit(1);
        }
        drop(stdin);

        let status = match child.wait() {
            Ok(status) => status,
            Err(error) => {
                eprintln!("Could not wait for 'tree' to finish: {error}");
                std::process::exit(1);
            }
        };
        match output_thread.join() {
            Ok(Ok(_)) => {}
            Ok(Err(error)) => {
                eprintln!("Could not write 'tree' output to stderr: {error}");
                std::process::exit(1);
            }
            Err(_) => {
                eprintln!("Could not forward 'tree' output to stderr");
                std::process::exit(1);
            }
        }

        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    }
}
