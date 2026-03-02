use inquire::{InquireError, Text};

use crate::utils::styles::{CLEAR_RENDER_CONFIG, SKIP_PLACEHOLDER, answered, muted};

pub fn prompt() -> Result<String, InquireError> {
    let message = Text::new("Body:")
        .with_render_config(*CLEAR_RENDER_CONFIG)
        .with_formatter(&|s| {
            if s.len() == 0 {
                return muted("<no body>");
            }
            answered(s)
        })
        .with_placeholder(&SKIP_PLACEHOLDER)
        .prompt();

    message
}
