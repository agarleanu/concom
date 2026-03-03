use std::fmt::{self, Display};

use crossterm::style::Stylize;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use inquire::{InquireError, Select};

use crate::{
    generated::{GITMOJIS, Gitmoji},
    utils::{
        styles::{CLEAR_RENDER_CONFIG, answered, muted},
        truncate::truncate_to_fit,
    },
};

impl Display for Gitmoji {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix_len = self.code.len() + 7;
        let desc = truncate_to_fit(self.description, prefix_len);
        let ellipsis = if desc.len() < self.description.len() {
            "…"
        } else {
            ""
        };

        let emoji = if !self.emoji.is_empty() {
            &format!("{} ", self.emoji)
        } else {
            ""
        };

        write!(
            f,
            "{} {}{}{}",
            self.code,
            emoji.bold(),
            desc.italic().dim(),
            ellipsis
        )
    }
}

pub fn prompt() -> Result<Option<Gitmoji>, InquireError> {
    let mut gitmojis = GITMOJIS.to_vec();

    gitmojis.insert(
        0,
        Gitmoji {
            emoji: "",
            entity: "",
            code: "None",
            description: "No gitmoji",
            name: "",
            semver: None,
        },
    );

    let gitmoji = Select::new("Gitmoji:", gitmojis)
        .with_render_config(*CLEAR_RENDER_CONFIG)
        .with_scorer(&|input, option, _string_value, _idx| -> Option<i64> {
            let matcher = SkimMatcherV2::default().ignore_case();
            matcher.fuzzy_match(
                &format!("{} {} {}", option.code, option.name, option.description),
                input,
            )
        })
        .with_formatter(&|a| {
            if a.value.emoji.is_empty() {
                return muted("<no gitmoji>");
            }
            answered(a.value.emoji)
        })
        .prompt()?;

    if gitmoji.emoji.is_empty() {
        return Ok(None);
    }

    Ok(Some(gitmoji))
}
