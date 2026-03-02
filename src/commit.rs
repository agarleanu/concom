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
