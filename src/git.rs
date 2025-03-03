use anyhow::{Context, Result};
use std::process::Command;

/// Get diff of staged changes
pub fn get_staged_diff() -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .context("Failed to execute git diff command")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to get staged diff: {}", error_message);
    }

    let diff = String::from_utf8(output.stdout).context("Parse git diff output failed")?;

    Ok(diff)
}
