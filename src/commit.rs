use crate::generated::{CommitType, Gitmoji};

pub struct ConventionalCommit {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub breaking: bool,
    pub gitmoji: Option<Gitmoji>,
    pub description: String,
    pub body: Option<String>,
    pub trailers: Vec<String>,
}

impl ConventionalCommit {
    pub fn header(&self) -> String {
        let mut s = self.commit_type.key.to_string();
        if let Some(scope) = &self.scope {
            s += &format!("({})", scope);
        }
        if self.breaking {
            s += "!";
        }
        s += ": ";
        if let Some(gitmoji) = &self.gitmoji {
            s += &format!("{} ", gitmoji.code);
        }
        s += &self.description;
        s
    }

    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec!["commit".to_string(), "-m".to_string(), self.header()];

        if let Some(body) = &self.body {
            args.extend(["-m".to_string(), body.clone()]);
        }

        for trailer in &self.trailers {
            args.extend(["--trailer".to_string(), trailer.clone()]);
        }

        args
    }
}

impl std::fmt::Display for ConventionalCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.header())?;
        if let Some(body) = &self.body {
            write!(f, "\n\n{}", body)?;
        }
        if !self.trailers.is_empty() {
            write!(f, "\n\n{}", self.trailers.join("\n"))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::Semver;
    use crate::utils::truncate::{truncate_to_fit, with_width};

    fn feat() -> CommitType {
        CommitType {
            key: "feat",
            title: "Features",
            description: "A new feature",
        }
    }

    fn sparkles() -> Gitmoji {
        Gitmoji {
            emoji: "✨",
            entity: "&#x2728;",
            code: ":sparkles:",
            description: "Introduce new features.",
            name: "sparkles",
            semver: Some(Semver::Minor),
        }
    }

    fn base() -> ConventionalCommit {
        ConventionalCommit {
            commit_type: feat(),
            scope: None,
            breaking: false,
            gitmoji: None,
            description: "add something".to_string(),
            body: None,
            trailers: vec![],
        }
    }

    #[test]
    fn header_minimal() {
        assert_eq!(base().header(), "feat: add something");
    }

    #[test]
    fn header_with_scope() {
        let c = ConventionalCommit {
            scope: Some("api".to_string()),
            ..base()
        };
        assert_eq!(c.header(), "feat(api): add something");
    }

    #[test]
    fn header_breaking_no_scope() {
        let c = ConventionalCommit {
            breaking: true,
            ..base()
        };
        assert_eq!(c.header(), "feat!: add something");
    }

    #[test]
    fn header_breaking_with_scope() {
        let c = ConventionalCommit {
            scope: Some("api".to_string()),
            breaking: true,
            ..base()
        };
        assert_eq!(c.header(), "feat(api)!: add something");
    }

    #[test]
    fn header_with_gitmoji() {
        let c = ConventionalCommit {
            gitmoji: Some(sparkles()),
            ..base()
        };
        assert_eq!(c.header(), "feat: :sparkles: add something");
    }

    #[test]
    fn header_full() {
        let c = ConventionalCommit {
            scope: Some("api".to_string()),
            breaking: true,
            gitmoji: Some(sparkles()),
            ..base()
        };
        assert_eq!(c.header(), "feat(api)!: :sparkles: add something");
    }

    #[test]
    fn args_minimal() {
        assert_eq!(
            base().to_args(),
            vec!["commit", "-m", "feat: add something"]
        );
    }

    #[test]
    fn args_with_body() {
        let c = ConventionalCommit {
            body: Some("Body text.".to_string()),
            ..base()
        };
        assert_eq!(
            c.to_args(),
            vec!["commit", "-m", "feat: add something", "-m", "Body text."]
        );
    }

    #[test]
    fn args_with_trailer() {
        let c = ConventionalCommit {
            trailers: vec!["BREAKING CHANGE: removed old API".to_string()],
            ..base()
        };
        assert_eq!(
            c.to_args(),
            vec![
                "commit",
                "-m",
                "feat: add something",
                "--trailer",
                "BREAKING CHANGE: removed old API"
            ]
        );
    }

    #[test]
    fn args_with_body_and_multiple_trailers() {
        let c = ConventionalCommit {
            body: Some("Body.".to_string()),
            trailers: vec![
                "BREAKING CHANGE: removed old API".to_string(),
                "Co-authored-by: Alice <alice@example.com>".to_string(),
            ],
            ..base()
        };
        assert_eq!(
            c.to_args(),
            vec![
                "commit",
                "-m",
                "feat: add something",
                "-m",
                "Body.",
                "--trailer",
                "BREAKING CHANGE: removed old API",
                "--trailer",
                "Co-authored-by: Alice <alice@example.com>",
            ]
        );
    }

    #[test]
    fn display_header_only() {
        assert_eq!(base().to_string(), "feat: add something");
    }

    #[test]
    fn display_with_body() {
        let c = ConventionalCommit {
            body: Some("Detailed explanation.".to_string()),
            ..base()
        };
        assert_eq!(
            c.to_string(),
            "feat: add something\n\nDetailed explanation."
        );
    }

    #[test]
    fn display_with_trailers() {
        let c = ConventionalCommit {
            trailers: vec!["BREAKING CHANGE: old API removed".to_string()],
            ..base()
        };
        assert_eq!(
            c.to_string(),
            "feat: add something\n\nBREAKING CHANGE: old API removed"
        );
    }

    #[test]
    fn display_multiple_trailers_joined_by_newline() {
        let c = ConventionalCommit {
            trailers: vec![
                "BREAKING CHANGE: old API removed".to_string(),
                "Co-authored-by: Alice <alice@example.com>".to_string(),
            ],
            ..base()
        };
        assert_eq!(
            c.to_string(),
            "feat: add something\n\nBREAKING CHANGE: old API removed\nCo-authored-by: Alice <alice@example.com>"
        );
    }

    #[test]
    fn display_full() {
        let c = ConventionalCommit {
            body: Some("Body.".to_string()),
            trailers: vec!["BREAKING CHANGE: old API removed".to_string()],
            ..base()
        };
        assert_eq!(
            c.to_string(),
            "feat: add something\n\nBody.\n\nBREAKING CHANGE: old API removed"
        );
    }

    #[test]
    fn commit_type_short_description_not_truncated() {
        with_width(80, || {
            assert_eq!(
                truncate_to_fit("A new feature", "feat".len() + 4),
                "A new feature"
            );
        });
    }

    #[test]
    fn commit_type_long_description_is_truncated() {
        with_width(30, || {
            let result =
                truncate_to_fit("Introduce styles into your application.", "feat".len() + 4);
            assert_eq!(result, "Introduce styles into ");
        });
    }

    #[test]
    fn gitmoji_short_description_not_truncated() {
        with_width(80, || {
            assert_eq!(
                truncate_to_fit("Introduce new features.", ":sparkles:".len() + 7),
                "Introduce new features."
            );
        });
    }

    #[test]
    fn gitmoji_long_description_is_truncated() {
        with_width(30, || {
            let result = truncate_to_fit("Introduce new features.", ":sparkles:".len() + 7);
            assert_eq!(result, "Introduce new");
        });
    }
}
