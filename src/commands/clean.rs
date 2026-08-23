use crate::commands::{Cli, RunCommand};

#[derive(Debug, clap::Args)]
pub struct CleanCommand {
    #[arg(short = 'y')]
    autoconfirm: bool,
}

impl RunCommand for CleanCommand {
    fn run(&self, root: &Cli) {
        println!("{root:?}");
    }
}
