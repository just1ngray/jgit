use crate::commands::{Cli, RunCommand};

#[derive(Debug, clap::Args)]
pub struct BranchCommand {
    /// The name of the branch to pull or create
    #[arg(required = true)]
    name: String,

    /// When creating a branch, this specifies which branch to split from
    #[arg(required = false, add = clap_complete::ArgValueCompleter::new(complete_branches))]
    from: String,
}

fn complete_branches(current: &std::ffi::OsStr) -> Vec<clap_complete::CompletionCandidate> {
    let mut completions = vec![];
    let current_str = current.to_str().unwrap_or("");

    // Shell out to git to get actual branch names
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
    fn run(&self, root: &Cli) {
        println!("{root:?}");
    }
}
