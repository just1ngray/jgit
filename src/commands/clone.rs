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
        let path = self.get_clone_path();
        if path.exists() {
            eprintln!("Path '{}' already exists", path.display());
            std::process::exit(1);
        }
        let git = crate::git::Git::new(root.git.clone(), path.clone());

        eprintln!(
            "Creating folder to hold jgit worktree repository at: {}",
            path.display()
        );
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|error| panic!("Could not create '{}': {error}", path.display()));

        git.run(["clone", "--bare", &self.url, ".bare"])
            .assert_success();
        std::fs::write(path.join(".git"), "gitdir: .bare\n").unwrap_or_else(|error| {
            panic!(
                "Could not create '{}': {error}",
                path.join(".git").display()
            )
        });
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
    fn get_clone_path(&self) -> PathBuf {
        // use if configured
        if let Some(p) = &self.path {
            return p.clone();
        }

        // derive from url
        return self.url
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".git"))
            .map(PathBuf::from)
            .expect(format!("Could not derive path from '{}'. Please pass a path explicitly. --help for detatils", &self.url).as_ref())
            .into();
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
            eprintln!("  Set globally:       $ git config --global user.email 'Your Name'");
            eprintln!(
                "  For this repo only: $ cd '$path' && git config --local user.email 'Your Name'"
            );
        }
        eprintln!("\x1b[0m");
    }
}
