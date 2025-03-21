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

/// Get the titles of the most recent commits
pub fn get_recent_commit_titles(count: usize) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args([
            "log",
            &format!("-{}", count),
            "--pretty=format:%s", // Only show commit subject
            "--no-merges",        // Exclude merge commits
            "--abbrev-commit",    // Abbreviate commit hashes
        ])
        .output()
        .context("Failed to execute git log command")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to get recent commit titles: {}", error_message);
    }

    let titles = String::from_utf8(output.stdout)
        .context("Parse git log output failed")?
        .lines()
        .map(String::from)
        .collect();

    Ok(titles)
}
