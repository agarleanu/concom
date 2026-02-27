pub fn term_width() -> usize {
    crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80)
}

pub fn truncate_to_fit(s: &str, prefix_visible_len: usize) -> &str {
    let available = term_width().saturating_sub(prefix_visible_len + 3);
    if s.len() <= available {
        return s;
    }
    let cutoff = s
        .char_indices()
        .map(|(i, _)| i)
        .take_while(|&i| i < available.saturating_sub(1))
        .last()
        .unwrap_or(0);
    &s[..cutoff]
}
