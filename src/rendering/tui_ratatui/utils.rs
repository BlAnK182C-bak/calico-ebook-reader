use crate::common::constants::{TUI_PADDING, TUI_PREFIX};

pub(super) fn parse_bottom_title(title_str: &str) -> String {
    format!(
        "{}{} | {}{}",
        " ".repeat(TUI_PADDING),
        title_str,
        TUI_PREFIX,
        " ".repeat(TUI_PADDING),
    )
}

pub(super) fn parse_top_title(title_str: &str) -> String {
    format!(
        "{}{}{}",
        " ".repeat(TUI_PADDING),
        title_str,
        " ".repeat(TUI_PADDING),
    )
}
