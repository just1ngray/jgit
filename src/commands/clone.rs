use std::path::PathBuf;

use crate::commands::{Cli, RunCommand};

#[derive(Debug, clap::Args)]
pub struct CloneCommand {
    /// The repository clone URL
    #[arg(required = true)]
    url: String,

    /// Directory in which to create the jgit repository
    #[arg(required = false, value_hint = clap::ValueHint::AnyPath)]
    path: Option<PathBuf>,
}

impl RunCommand for CloneCommand {
    fn run(&self, root: &Cli) {
        let path = match self.get_clone_path() {
            Ok(path) => path,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        };
        if path.exists() {
            eprintln!("Path '{}' already exists", path.display());
            std::process::exit(1);
        }
        let git = crate::git::Git::new(root.git.clone(), path.clone());

        eprintln!(
            "Creating folder to hold jgit worktree repository at: {}",
            path.display()
        );
        if let Err(error) = std::fs::create_dir_all(&path) {
            eprintln!("Could not create '{}': {error}", path.display());
            std::process::exit(1);
        }

        git.run(["clone", "--bare", &self.url, ".bare"])
            .assert_success();
        let git_file = path.join(".git");
        if let Err(error) = std::fs::write(&git_file, "gitdir: .bare\n") {
            eprintln!("Could not create '{}': {error}", git_file.display());
            std::process::exit(1);
        };
        git.run([
            "config",
            "remote.origin.fetch",
            "+refs/heads/*:refs/remotes/origin/*",
        ])
        .assert_success();

        self.print_config_warnings(&git);

        println!("{}", path.display());
    }
}

impl CloneCommand {
    fn get_clone_path(&self) -> Result<PathBuf, String> {
        if self.url.is_empty() {
            return Err("Usage: clone <url> [path]".to_string());
        }

        // use if configured
        if let Some(path) = &self.path {
            if path.as_os_str().is_empty() {
                return Err("Usage: clone <url> [path]".to_string());
            }
            return Ok(path.clone());
        }

        // derive from url
        self.url
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".git"))
            .filter(|name| !name.is_empty())
            .map(PathBuf::from)
            .ok_or_else(|| {
                format!(
                    "Could not derive path from '{}'. Please pass a path explicitly.",
                    self.url
                )
            })
    }

    fn print_config_warnings(&self, git: &crate::git::Git) {
        eprintln!("\x1b[38;5;208m");
        if git.run(["config", "--get", "user.name"]).rc != 0 {
            eprintln!(
                "WARNING! Git config doesn't know your user.name. You won't be able to commit unless you configure it."
            );
            eprintln!("  Set globally:       $ git config --global user.name 'Your Name'");
            eprintln!(
                "  For this repo only: $ cd '$path' && git config --local user.name 'Your Name'"
            );
        }
        if git.run(["config", "--get", "user.email"]).rc != 0 {
            eprintln!(
                "WARNING! Git config doesn't know your user.email. You won't be able to commit unless you configure it."
            );
            eprintln!("  Set globally:       $ git config --global user.email 'email@example.com'");
            eprintln!(
                "  For this repo only: $ cd '$path' && git config --local user.email 'email@example.com'"
            );
        }
        eprintln!("\x1b[0m");
    }
}
