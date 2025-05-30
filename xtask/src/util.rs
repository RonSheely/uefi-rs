// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::{Result, bail};
use std::process::Command;

/// Format a `Command` as a `String.
///
/// Example: "VAR=val program --arg1 arg2".
pub fn command_to_string(cmd: &Command) -> String {
    // Format env vars as "name=val".
    let mut parts = cmd
        .get_envs()
        // Filter out variables that are set or cleared by
        // `fix_nested_cargo_env`, as they would clutter the output.
        .filter(|(name, val)| {
            *name != "PATH"
                // Exclude any variables cleared with `Command::env_remove`.
                && val.is_some()
        })
        .map(|(name, val)| {
            format!(
                "{}={}",
                name.to_string_lossy(),
                val.unwrap_or_default().to_string_lossy()
            )
        })
        .collect::<Vec<_>>();

    // Add the program name.
    parts.push(cmd.get_program().to_string_lossy().to_string());

    // Add each argument.
    parts.extend(cmd.get_args().map(|arg| arg.to_string_lossy().to_string()));

    // Join the vars, program, and arguments into a single string.
    parts.into_iter().collect::<Vec<_>>().join(" ")
}

/// Print a `Command` and run it, then check that it completes
/// successfully.
pub fn run_cmd(mut cmd: Command) -> Result<()> {
    println!("run_cmd: '{}'", command_to_string(&cmd));

    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        bail!("command failed: {}", status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_to_string() {
        let mut cmd = Command::new("MyCommand");
        cmd.args(["abc", "123"])
            .envs([("VAR1", "val1"), ("VAR2", "val2"), ("PATH", "pathval")])
            .env_remove("RUSTC")
            .env_remove("CARGO");
        assert_eq!(
            command_to_string(&cmd),
            "VAR1=val1 VAR2=val2 MyCommand abc 123"
        );
    }
}
