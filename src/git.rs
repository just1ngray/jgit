pub struct Git {
    exec: String,
    cwd: std::path::PathBuf,
}

impl Git {
    pub fn new(exec: String, cwd: std::path::PathBuf) -> Self {
        return Git { exec, cwd };
    }

    pub fn run<const N: usize>(&self, args: [&str; N]) -> GitResult {
        let command = format!(
            "{} $ {} {}",
            self.cwd.to_string_lossy(),
            self.exec,
            args.join(" ")
        );
        eprintln!("{command}");
        let res = std::process::Command::new(&self.exec)
            .args(args)
            .current_dir(&self.cwd)
            .output();

        // raise an error if there's a problem with the executable itself
        if let Err(error) = res {
            eprintln!("Could not execute '{}': {}", self.exec, error);
            std::process::exit(1);
        }
        let output = res.unwrap();

        // program ran; sanitize into result
        let rc = output.status.code().unwrap_or(1);
        let stderr = String::from_utf8(output.stderr).expect("stderr not utf-8");
        for line in stderr.lines() {
            eprintln!("{line}");
        }

        let stdout = String::from_utf8(output.stdout).expect("stderr not utf-8");
        return GitResult {
            rc,
            stderr,
            stdout,
            command,
        };
    }
}

#[must_use = "Git commands can fail; explicitly call .or_true() or .assert_success()"]
pub struct GitResult {
    pub rc: i32,
    pub stderr: String,
    pub stdout: String,
    pub command: String,
}

impl GitResult {
    pub fn assert_success(self) {
        if self.rc != 0 {
            eprintln!(
                "Exiting {} after failed git command: {}\n{}",
                self.rc, self.command, self.stderr,
            );
            std::process::exit(self.rc);
        }
    }
}
