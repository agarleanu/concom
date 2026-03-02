use std::fmt::{self, Display};

use crossterm::style::Stylize;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use inquire::{InquireError, Select};

use crate::{
    generated::{COMMIT_TYPES, CommitType},
    utils::{
        styles::{BASE_RENDER_CONFIG, answered},
        truncate::truncate_to_fit,
    },
};

impl Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix_len = self.key.len() + 1; // key + space
        let desc = truncate_to_fit(self.description, prefix_len);
        let ellipsis = if desc.len() < self.description.len() {
            "…"
        } else {
            ""
        };
        write!(f, "{} {}{}", self.key.bold(), desc.italic().dim(), ellipsis)
    }
}

pub fn prompt() -> Result<CommitType, InquireError> {
    let commit_type: Result<CommitType, InquireError> =
        Select::new("Change type:", COMMIT_TYPES.to_vec())
            .with_render_config(*BASE_RENDER_CONFIG)
            .with_scorer(&|input, option, _string_value, _idx| -> Option<i64> {
                let matcher = SkimMatcherV2::default().ignore_case();
                matcher.fuzzy_match(
                    &format!("{} {} {}", option.key, option.title, option.description),
                    input,
                )
            })
            .with_formatter(&|a| answered(a.value.key))
            .prompt();

    commit_type
}
