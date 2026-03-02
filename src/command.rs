use std::process::Command;

use crate::commit::ConventionalCommit;

pub fn run_git_commit(commit: &ConventionalCommit) -> std::io::Result<()> {
    Command::new("git").args(commit.to_args()).status()?;
    Ok(())
}

pub fn copy_to_clipboard(text: &str) -> Result<(), arboard::Error> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}
