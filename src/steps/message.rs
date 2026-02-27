use inquire::{InquireError, Text, validator::MinLengthValidator};

pub fn prompt() -> Result<String, InquireError> {
    let message: Result<String, InquireError> =
        Text::new("Write a short, imperative tense description of the change.")
            .with_validator(
                MinLengthValidator::new(1).with_message("Commit message must be non-empty!"),
            )
            .prompt();

    message
}
