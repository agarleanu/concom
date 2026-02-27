use std::fmt::{self, Display};

use crossterm::style::Stylize;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use inquire::{InquireError, Select};

use crate::{
    generated::{GITMOJIS, Gitmoji},
    utils::truncate::truncate_to_fit,
};

impl Display for Gitmoji {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix_len = self.code.len() + 3; // code + space + emoji + space
        let desc = truncate_to_fit(self.description, prefix_len);
        let ellipsis = if desc.len() < self.description.len() {
            "…"
        } else {
            ""
        };
        write!(
            f,
            "{} {} {}{}",
            self.code,
            self.emoji,
            desc.italic().dim(),
            ellipsis
        )
    }
}

pub fn prompt() -> Result<Option<Gitmoji>, InquireError> {
    let gitmoji = Select::new(
        "Select the type of change that you're committing.",
        GITMOJIS.to_vec(),
    )
    .with_scorer(&|input, option, _string_value, _idx| -> Option<i64> {
        let matcher = SkimMatcherV2::default().ignore_case();
        matcher.fuzzy_match(
            format!("{} {} {}", option.code, option.name, option.description).as_str(),
            input,
        )
    })
    .with_formatter(&|a| format!("{}", a.value.emoji))
    .prompt_skippable();

    gitmoji
}
