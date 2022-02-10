use std::io::Result;
use std::process::Command;
use std::process::ExitStatus;
use blocking::unblock;

pub async fn directory_exists(path: String) -> bool {
    unblock(|| std::fs::read_dir(path).is_ok()).await
}

pub async fn file_exists(path: String) -> bool {
    unblock(|| std::fs::read(path).is_ok()).await
}

pub async fn run_command(cmd: String) -> Result<ExitStatus> {
    run_command_with_path(cmd, "./".to_string()).await
}

pub async fn run_command_with_path(cmd: String, path: String) -> Result<ExitStatus> {
    unblock(|| {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(path)
            .status()
    }).await
}
