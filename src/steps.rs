use inquire::InquireError;

use crate::generated::{CommitType, Gitmoji};

pub mod conv_type;
pub mod gitmoji;
pub mod message;
pub mod scope;

fn compose(t: CommitType, s: String, g: Option<Gitmoji>, m: String) -> String {
    let mut commit: String = t.key.to_string();

    if s.len() > 0 {
        commit += format!("({})", s).as_str();
    }

    // TODO: Add breaking change handling here

    commit += ": ";

    if let Some(gitmoji) = g {
        commit += format!("{} ", gitmoji.code).as_str();
    }

    commit += m.as_str();

    commit
}

pub fn run() -> Result<String, InquireError> {
    let t = conv_type::prompt()?;
    let s = scope::prompt()?;
    let g = gitmoji::prompt()?;
    let m = message::prompt()?;

    Ok(compose(t, s, g, m))
}
