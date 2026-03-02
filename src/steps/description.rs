use inquire::{InquireError, Text, validator::MinLengthValidator};

use crate::utils::styles::{BASE_RENDER_CONFIG, answered};

pub fn prompt() -> Result<String, InquireError> {
    let message = Text::new("Short description:")
        .with_formatter(&|a| answered(a))
        .with_render_config(*BASE_RENDER_CONFIG)
        .with_validator(
            MinLengthValidator::new(1).with_message("Commit message must be non-empty!"),
        )
        .prompt();

    message
}
