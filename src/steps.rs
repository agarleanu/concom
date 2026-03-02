use std::error::Error;

use crate::command;
use crate::commit::ConventionalCommit;

pub mod action;
pub mod body;
pub mod breaking_change;
pub mod conv_type;
pub mod description;
pub mod gitmoji;
pub mod scope;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut trailers: Vec<String> = vec![];

    let commit_type = conv_type::prompt()?;
    let scope = scope::prompt()?;
    let gitmoji = gitmoji::prompt()?;
    let description = description::prompt()?;
    let body = body::prompt()?;
    let breaking_change = breaking_change::prompt()?;

    if let Some(message) = &breaking_change
        && !message.is_empty()
    {
        trailers.push(format!("BREAKING CHANGE: {}", message));
    }

    let commit = ConventionalCommit {
        commit_type,
        scope: if scope.is_empty() { None } else { Some(scope) },
        breaking: breaking_change.is_some(),
        gitmoji,
        description,
        body: if body.is_empty() { None } else { Some(body) },
        trailers,
    };

    match action::prompt()? {
        action::Action::Commit => command::run_git_commit(&commit)?,
        action::Action::Copy => command::copy_to_clipboard(&commit.to_string())?,
        action::Action::Print => println!("{}", commit),
    }

    Ok(())
}
