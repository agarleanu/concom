use std::fmt::{self, Display};

use inquire::{InquireError, Select};

use crate::utils::styles::{BASE_RENDER_CONFIG, answered};

pub enum Action {
    Commit,
    Copy,
    Print,
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Commit => write!(f, "Commit"),
            Action::Copy => write!(f, "Copy message to clipboard"),
            Action::Print => write!(f, "Print message"),
        }
    }
}

pub fn prompt() -> Result<Action, InquireError> {
    Select::new("Action:", vec![Action::Commit, Action::Copy, Action::Print])
        .with_formatter(&|s| answered(format!("{}", s.value)))
        .with_render_config(*BASE_RENDER_CONFIG)
        .prompt()
}
