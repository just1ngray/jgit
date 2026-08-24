use crate::commands::{JGitCli, RunCommand};

#[derive(Debug, clap::Args)]
pub struct BranchCommand {
    /// The name of the branch to pull or create
    #[arg(required = true)]
    name: String,

    /// When creating a branch, this specifies which branch to split from
    #[arg(required = false, add = clap_complete::ArgValueCompleter::new(complete_branches))]
    from: Option<String>,
}

fn complete_branches(current: &std::ffi::OsStr) -> Vec<clap_complete::CompletionCandidate> {
    let mut completions = vec![];
    let current_str = current.to_str().unwrap_or("");

    // Shell out to git to get actual branch names
    // noted limitation: typically we permit ``--git /executable`` flag but this is
    // not supported for auto completions because we don't know the flag's value
    if let Ok(output) = std::process::Command::new("git")
        .arg("branch")
        .arg("--format=%(refname:short)")
        .output()
    {
        if let Ok(branches) = String::from_utf8(output.stdout) {
            for branch in branches.lines() {
                let branch = branch.trim();
                // Filter by what the user has already typed
                if branch.starts_with(current_str) {
                    completions.push(clap_complete::CompletionCandidate::new(branch));
                }
            }
        }
    }

    return completions;
}

impl RunCommand for BranchCommand {
    fn run(&self, root: &JGitCli) {
        let cwd = std::env::current_dir().expect("Could not determine current directory");
        if !cwd.join(".bare").exists() {
            eprintln!("This command must be executed from git repository");
            std::process::exit(1);
        }

        let branch_path = cwd.join(&self.name);
        if branch_path.exists() {
            eprintln!(
                "Branch '{}' already exists as a local worktree directory",
                self.name
            );
            std::process::exit(1);
        }

        let git = crate::git::Git::new(root.git.clone(), cwd.clone());
        git.run(["fetch"]).assert_success();

        let exists_on_remote = git
            .run(["ls-remote", "--exit-code", "origin", &self.name])
            .rc
            == 0;

        if exists_on_remote {
            git.run(["worktree", "add", "-B", &self.name, &self.name])
                .assert_success();

            let worktree_git = crate::git::Git::new(root.git.clone(), branch_path.clone());
            worktree_git
                .run([
                    "branch",
                    "--set-upstream-to",
                    &format!("origin/{}", self.name),
                ])
                .assert_success();
            worktree_git
                .run(["reset", "--hard", &format!("origin/{}", self.name)])
                .assert_success();
        } else {
            eprintln!("Branch '{}' does not exist on remote", self.name);

            let from_branch = self.get_from_branch(&git);
            if git
                .run(["ls-remote", "--exit-code", "origin", &from_branch])
                .rc
                != 0
            {
                eprintln!("Err: Branch '{}' does not exist on remote", from_branch);
                std::process::exit(1);
            }

            eprintln!("Creating branch '{}' from '{}'", self.name, from_branch);
            git.run(["fetch", "origin", &from_branch]).assert_success();
            git.run([
                "worktree",
                "add",
                "-B",
                &self.name,
                &self.name,
                &format!("origin/{from_branch}"),
            ])
            .assert_success();

            let worktree_git = crate::git::Git::new(root.git.clone(), branch_path.clone());
            worktree_git
                .run(["push", "origin", &self.name])
                .assert_success();
            worktree_git
                .run([
                    "branch",
                    "--set-upstream-to",
                    &format!("origin/{}", self.name),
                ])
                .assert_success();
        }

        println!("{}", self.name);
    }
}

impl BranchCommand {
    fn get_from_branch(&self, git: &crate::git::Git) -> String {
        if let Some(from) = &self.from {
            return from.clone();
        }

        self.get_branch_interactive(&self.get_main_branch(git))
    }

    fn get_main_branch(&self, git: &crate::git::Git) -> String {
        let head = git.run(["symbolic-ref", "HEAD"]);
        if head.rc != 0 {
            eprintln!("Could not determine current branch: {}", head.stderr);
            std::process::exit(head.rc);
        }

        head.stdout
            .trim()
            .strip_prefix("refs/heads/")
            .unwrap_or(head.stdout.trim())
            .to_string()
    }

    fn get_branch_interactive(&self, default_branch: &str) -> String {
        use std::io::Write;
        eprint!(
            "Create branch '{}' from which branch? ({}): ",
            self.name, default_branch
        );
        std::io::stderr().flush().ok();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");
        let input = input.trim();

        if input.is_empty() {
            default_branch.to_string()
        } else {
            input.to_string()
        }
    }
}
