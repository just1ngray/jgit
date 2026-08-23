use crate::commands::{Cli, RunCommand};



#[derive(Debug, clap::Args)]
pub struct TreeCommand {
    /// Disable printing branches and only show worktrees
    #[arg(short = 'b')]
    hide_branches: bool,
}

impl RunCommand for TreeCommand {
    fn run(&self, root: &Cli) {
        println!("{root:?}");
    }
}
