extern crate git2;

use std::process::Command;
use std::path::PathBuf;

pub struct Shell
{
    pub cwd: PathBuf,
}

impl Shell
{
    pub fn command(&self, cmd: &str) -> String
    {
        debug!("Shell::command: {}", cmd);
        let output = Command::new("sh")
            .current_dir(&self.cwd)
            .arg("-c")
            .arg(&cmd)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        return String::from_utf8(output.stdout).expect("failed to decode utf8").trim().to_string();
    }
}
