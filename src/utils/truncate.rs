use unicode_truncate::UnicodeTruncateStr;

pub fn term_width() -> usize {
    #[cfg(test)]
    if let Some(w) = MOCK_WIDTH.with(|c| c.get()) {
        return w;
    }
    crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80)
}

pub fn truncate_to_fit(s: &str, prefix_visible_len: usize) -> &str {
    let available = term_width().saturating_sub(prefix_visible_len);
    s.unicode_truncate(available).0
}

#[cfg(test)]
use std::cell::Cell;

#[cfg(test)]
thread_local! {
    static MOCK_WIDTH: Cell<Option<usize>> = Cell::new(None);
}

#[cfg(test)]
pub fn with_width<F: FnOnce()>(width: usize, f: F) {
    MOCK_WIDTH.with(|c| c.set(Some(width)));
    f();
    MOCK_WIDTH.with(|c| c.set(None));
}
