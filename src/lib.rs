use std::io::{Stderr, Stdin, Stdout};

pub mod message;

pub struct Fds {
    pub stdin: Stdin,
    pub stdout: Stdout,
    pub stderr: Stderr,
}

impl Default for Fds {
    fn default() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            stderr: std::io::stderr(),
        }
    }
}
