use std::io::Write;

use crate::commands::{JGitCli, RunCommand};

#[derive(Debug, clap::Subcommand)]
enum Operations {
    /// Generates the auto-complete script for the selected shell without adding it
    Generate,

    /// Installs the auto-complete script into the selected shell
    Install,

    /// Uninstalls the auto-complete script from the selected shell
    Uninstall,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Shell {
    Bash,
    Fish,
    Zsh,
}

#[derive(Debug, clap::Args)]
pub struct AutocompleteCommand {
    #[arg(required = false, short, long, value_enum)]
    shell: Option<Shell>,

    #[command(subcommand)]
    operation: Operations,
}

impl RunCommand for AutocompleteCommand {
    fn run(&self, _root: &JGitCli) {
        let shell = self.shell.unwrap_or_else(|| Self::get_current_shell());

        match (&self.operation, shell) {
            (Operations::Generate, Shell::Bash) => println!("{}", BASH),
            (Operations::Generate, Shell::Zsh) => println!("{}", ZSH),
            (Operations::Generate, Shell::Fish) => println!("{}", FISH),
            (Operations::Install, Shell::Bash) => Self::append_idempotent_in_home(".bashrc", BASH),
            (Operations::Install, Shell::Zsh) => Self::append_idempotent_in_home(".zshrc", ZSH),
            (Operations::Install, Shell::Fish) => Self::install_fish(),
            (Operations::Uninstall, Shell::Bash) => Self::uninstall_from_home(".bashrc", BASH),
            (Operations::Uninstall, Shell::Zsh) => Self::uninstall_from_home(".zshrc", ZSH),
            (Operations::Uninstall, Shell::Fish) => Self::uninstall_fish(),
        };
    }
}

impl AutocompleteCommand {
    fn get_current_shell() -> Shell {
        let shell_path = std::env::var("SHELL").expect("SHELL environment variable is not set");

        let shell_name = shell_path.rsplit('/').next().unwrap_or(&shell_path);

        return match shell_name {
            "bash" => Shell::Bash,
            "fish" => Shell::Fish,
            "zsh" => Shell::Zsh,
            other => {
                eprintln!("Unsupported shell: {other}");
                std::process::exit(1);
            },
        };
    }

    fn install_fish() {
        let path = home::home_dir()
            .expect("Unknown home directory")
            .join(".config/fish/conf.d/jgit.fish");

        if let Some(parent) = path.parent() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }

        match std::fs::write(&path, FISH) {
            Ok(_) => {
                eprintln!("Added jgit install to {}", &path.display());
                eprintln!("Run 'source {}' now", &path.display());
            }
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };
    }

    fn append_idempotent_in_home(path_in_homedir: &str, content: &str) {
        let path = home::home_dir()
            .expect("Unknown home directory")
            .join(path_in_homedir);

        if let Ok(file_content) = std::fs::read_to_string(&path) {
            if file_content.contains(content) {
                eprintln!("Already installed in {}", &path.display());
                return;
            }
        }

        if let Some(parent) = path.parent() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect(format!("Could not open {}", path.display()).as_ref());

        match writeln!(file, "{}", content) {
            Ok(_) => {
                eprintln!("Added jgit install to {}", &path.display());
                eprintln!("Run 'source {}' now", &path.display());
            }
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };
    }

    fn uninstall_fish() {
        let path = home::home_dir()
            .expect("Unknown home directory")
            .join(".config/fish/conf.d/jgit.fish");

        if path.exists() {
            match std::fs::remove_file(&path) {
                Ok(_) => {
                    eprintln!("Uninstalled jgit from {}", &path.display());
                }
                Err(error) => {
                    eprintln!("{}", error);
                    std::process::exit(1);
                }
            };
        } else {
            eprintln!(
                "Already uninstalled from {} (does not exist)",
                &path.display()
            );
        }
    }

    fn uninstall_from_home(path_in_homedir: &str, content: &str) {
        let path = home::home_dir()
            .expect("Unknown home directory")
            .join(path_in_homedir);

        if let Ok(file_content) = std::fs::read_to_string(&path) {
            let new = file_content.replace(&content, "");
            if new != file_content {
                match std::fs::write(&path, new) {
                    Ok(_) => {
                        eprintln!("Uninstalled jgit from {}", &path.display());
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                        eprintln!();
                        std::process::exit(1);
                    }
                };
            } else {
                eprintln!("Already uninstalled from {}", &path.display());
            }
        } else {
            eprintln!(
                "Already uninstalled from {} (does not exist)",
                &path.display()
            );
        }
    }
}

/// Add to ~/.bashrc
const BASH: &'static str = "#> Added by jgit install (~/.bashrc)
source <(COMPLETE=bash jgit)
#< end";

/// Add to ~/.zshrc
const ZSH: &'static str = "#> Added by jgit install (~/.zshrc)
source <(COMPLETE=zsh jgit)
#< end";

/// Add to ~/.config/fish/conf.d/jgit.fish
const FISH: &'static str = "# Added by jgit install (~/.config/fish/conf.d/jgit.fish)
if status is-interactive
  COMPLETE=fish jgit | source
end";
