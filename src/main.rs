use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

mod commands;
mod git;

fn main() {
    // source-based in-place autocompletion evaluation
    // see crate::commands::autocomplete for more info
    CompleteEnv::with_factory(commands::Cli::command).complete();

    let cli = commands::Cli::parse();
    cli.run();
}
