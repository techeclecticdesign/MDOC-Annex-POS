use crate::common::error::AppError;
use std::process::{Command, Output};

pub struct WindowsCommandRunner;

pub trait CommandRunner: Send + Sync {
    fn run(&self, program: &str, args: &[&str]) -> Result<Output, AppError>;
}

impl CommandRunner for WindowsCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<Output, AppError> {
        Command::new(program)
            .args(args)
            .output()
            .map_err(|e| AppError::Unexpected(format!("Failed to run command: {e}")))
    }
}
