use std::path::{Path, PathBuf};

use termtree::Tree;

use crate::commands::{JGitCli, RunCommand};

#[derive(Debug, clap::Args)]
pub struct TreeCommand {
    /// Only show jgit repositories, hiding their worktrees
    #[arg(short = 'b')]
    hide_branches: bool,
}

impl RunCommand for TreeCommand {
    fn run(&self, root: &JGitCli) {
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

        let mut tree = Tree::new(".".to_string());
        let mut worktree_count = 0;

        for repo in &repos {
            let repo_components = Self::path_components(repo);
            Self::add_path(&mut tree, repo_components.iter().cloned());

            if !show_worktrees {
                continue;
            }

            let repo_path = cwd.join(repo);
            let repo_abs = std::fs::canonicalize(&repo_path).unwrap_or(repo_path.clone());
            let git = crate::git::Git::new(root.git.clone(), repo_path);
            let worktrees = Self::get_worktrees(&git);

            if worktrees.is_empty() {
                Self::add_path(
                    &mut tree,
                    repo_components
                        .iter()
                        .cloned()
                        .chain(std::iter::once("(no worktrees)".to_string())),
                );
            } else {
                for worktree in &worktrees {
                    worktree_count += 1;
                    let relative_worktree = worktree.strip_prefix(&repo_abs).unwrap_or(worktree);
                    let worktree_name = relative_worktree.display().to_string().replace('/', "∕");
                    Self::add_path(
                        &mut tree,
                        repo_components
                            .iter()
                            .cloned()
                            .chain(std::iter::once(worktree_name)),
                    );
                }
            }
        }

        print!("{tree}");
        println!();
        if show_worktrees {
            println!(
                "{} jgit repositories, {worktree_count} worktrees",
                repos.len()
            );
        } else {
            println!("{} jgit repositories", repos.len());
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

    fn path_components(path: &Path) -> Vec<String> {
        if path == Path::new(".") {
            return vec![];
        }

        path.components()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .collect()
    }

    fn add_path(tree: &mut Tree<String>, components: impl IntoIterator<Item = String>) {
        let mut node = tree;
        for component in components {
            let child_index = node
                .leaves
                .iter()
                .position(|child| child.root == component)
                .unwrap_or_else(|| {
                    node.leaves.push(Tree::new(component));
                    node.leaves.len() - 1
                });
            node = &mut node.leaves[child_index];
        }
    }
}
