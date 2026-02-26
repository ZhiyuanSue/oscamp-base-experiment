//! # Process and Pipes
//!
//! In this exercise, you will learn how to create child processes and communicate through pipes.
//!
//! ## Concepts
//! - `std::process::Command` creates child processes
//! - `Stdio::piped()` sets up pipes
//! - Communicate with child processes via stdin/stdout
//! - Obtain child process exit status

use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Execute the given shell command and return its stdout output.
///
/// For example: `run_command("echo", &["hello"])` should return `"hello\n"`
pub fn run_command(program: &str, args: &[&str]) -> String {
    // TODO: Use Command::new to create process
    // TODO: Set stdout to Stdio::piped()
    // TODO: Execute with .output() and get output
    // TODO: Convert stdout to String and return
    todo!()
}

/// Write data to child process (cat) stdin via pipe and read its stdout output.
///
/// This demonstrates bidirectional pipe communication between parent and child processes.
pub fn pipe_through_cat(input: &str) -> String {
    // TODO: Create "cat" command, set stdin and stdout to piped
    // TODO: Spawn process
    // TODO: Write input to child process stdin
    // TODO: Drop stdin to close pipe (otherwise cat won't exit)
    // TODO: Read output from child process stdout
    todo!()
}

/// Get child process exit code.
/// Execute command `sh -c {command}` and return the exit code.
pub fn get_exit_code(command: &str) -> i32 {
    // TODO: Use Command::new("sh").args(["-c", command])
    // TODO: Execute and get status
    // TODO: Return exit code
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_echo() {
        let output = run_command("echo", &["hello"]);
        assert_eq!(output.trim(), "hello");
    }

    #[test]
    fn test_run_with_args() {
        let output = run_command("echo", &["-n", "no newline"]);
        assert_eq!(output, "no newline");
    }

    #[test]
    fn test_pipe_cat() {
        let output = pipe_through_cat("hello pipe!");
        assert_eq!(output, "hello pipe!");
    }

    #[test]
    fn test_pipe_multiline() {
        let input = "line1\nline2\nline3";
        assert_eq!(pipe_through_cat(input), input);
    }

    #[test]
    fn test_exit_code_success() {
        assert_eq!(get_exit_code("true"), 0);
    }

    #[test]
    fn test_exit_code_failure() {
        assert_eq!(get_exit_code("false"), 1);
    }
}
