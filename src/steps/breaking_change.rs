use inquire::{Confirm, InquireError, Text};

use crate::utils::styles::{BASE_RENDER_CONFIG, SKIP_PLACEHOLDER, answered, muted};

pub fn prompt() -> Result<Option<String>, InquireError> {
    let breaking = Confirm::new("Breaking change?")
        .with_formatter(&|a| {
            if a {
                answered("Yes")
            } else {
                muted("<no breaking change>")
            }
        })
        .with_render_config(*BASE_RENDER_CONFIG)
        .with_default(false)
        .prompt()?;

    if breaking {
        let message = Text::new("Breaking change description:")
            .with_render_config(*BASE_RENDER_CONFIG)
            .with_formatter(&|a| {
                if a.is_empty() {
                    return muted("<no body>");
                }
                answered(a)
            })
            .with_placeholder(&SKIP_PLACEHOLDER)
            .prompt()?;

        return Ok(Some(message));
    }

    Ok(None)
}
