use crossterm::style::Stylize;
use inquire::{
    InquireError, Text,
    ui::{Attributes, RenderConfig},
};

pub fn prompt() -> Result<String, InquireError> {
    let mut scope_render_config = RenderConfig::default_colored();
    scope_render_config.answer.att = Attributes::empty();
    scope_render_config.answer.fg = None;

    let scope: Result<String, InquireError> = Text::new("Select the scope of this change.")
        .with_render_config(scope_render_config)
        .with_formatter(&|s| {
            if s.len() == 0 {
                return "<no scope>".italic().dim().to_string();
            }
            s.cyan().to_string()
        })
        .prompt();

    scope
}
