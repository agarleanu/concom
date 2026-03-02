use crossterm::style::Stylize;
use inquire::ui::{Attributes, RenderConfig, StyleSheet, Styled};
use std::cell::LazyCell;

pub const BASE_RENDER_CONFIG: LazyCell<RenderConfig> = LazyCell::new(|| {
    let mut rc = RenderConfig::default();

    rc.prompt_prefix = Styled::new("◯")
        .with_style_sheet(StyleSheet::new().with_fg(inquire::ui::Color::rgb(250, 102, 115)))
        .with_attr(Attributes::BOLD);
    rc.scroll_up_prefix = Styled::new("↑").with_attr(Attributes::BOLD);
    rc.scroll_down_prefix = Styled::new("↓").with_attr(Attributes::BOLD);
    rc.highlighted_option_prefix = Styled::new("→").with_attr(Attributes::BOLD);
    rc.selected_option = Some(StyleSheet::new().with_fg(inquire::ui::Color::LightRed));
    rc.answered_prompt_prefix = Styled::new("●")
        .with_style_sheet(StyleSheet::new().with_fg(inquire::ui::Color::rgb(250, 102, 115)));

    rc
});

pub const CLEAR_RENDER_CONFIG: LazyCell<RenderConfig> = LazyCell::new(|| {
    let mut rc = BASE_RENDER_CONFIG.clone();
    rc.answer.att = Attributes::empty();
    rc.answer.fg = None;

    rc
});

pub fn muted(s: impl Into<String>) -> String {
    s.into().grey().dim().italic().to_string()
}

pub fn answered(s: impl Into<String>) -> String {
    s.into().blue().to_string()
}

pub const SKIP_PLACEHOLDER: LazyCell<String> = LazyCell::new(|| {
    format!(
        "{} {} {}",
        muted("Press"),
        muted("[↲ enter]").bold(),
        muted("to skip")
    )
});
