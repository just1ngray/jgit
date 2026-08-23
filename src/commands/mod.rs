mod autocomplete;
mod branch;
mod clean;
mod clone;
mod tree;

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// Recursively finds and prints jgit worktree repositories
    Tree(tree::TreeCommand),

    /// Clone a new jgit-managed git repository
    Clone(clone::CloneCommand),

    /// Create a new jgit worktree for a branch name
    Branch(branch::BranchCommand),

    /// Delete local worktrees that do not have corresponding remote branches,
    /// and branches not checked out by any worktree
    Clean(clean::CleanCommand),

    /// Generate, add, and remove jgit autocompletions for your shell
    Autocomplete(autocomplete::AutocompleteCommand),
}

#[derive(clap::Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, value_hint = clap::ValueHint::ExecutablePath, default_value = "git")]
    pub git: String,
}

pub trait RunCommand {
    fn run(&self, root: &Cli);
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Tree(cmd) => cmd.run(&self),
            Commands::Clone(cmd) => cmd.run(&self),
            Commands::Branch(cmd) => cmd.run(&self),
            Commands::Clean(cmd) => cmd.run(&self),
            Commands::Autocomplete(cmd) => cmd.run(&self),
        }
    }
}
