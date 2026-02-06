use std::io::{self, Write};

use crossterm::terminal;
use log::warn;

/// Render markdown text to the terminal using termimad.
pub fn render_markdown(text: &str) {
    // termimad::print_text renders markdown with formatting
    // (bold, headers, code blocks, lists, etc.)
    termimad::print_text(text);
}

/// Try to clear the previously printed raw text from the terminal.
///
/// Calculates how many terminal lines the text occupied (accounting for
/// line wrapping at terminal width). If it fits within the terminal height,
/// moves the cursor up and clears. Returns `true` if cleared, `false` if
/// the text was too long (scrolled off-screen).
pub(crate) fn try_clear_lines(text: &str) -> bool {
    let (terminal_width, terminal_height) = match terminal::size() {
        Ok((w, h)) => (w as usize, h as usize),
        Err(e) => {
            warn!("Failed to get terminal size: {}", e);
            return false;
        }
    };

    let total_lines: usize = text
        .split('\n')
        .map(|line| (line.len() / terminal_width) + 1)
        .sum();

    if total_lines >= terminal_height {
        return false;
    }

    //  - "\x1b[{total_lines}A" moves cursor up
    //  - "\x1b[J" clears from cursor to end of screen
    print!("\x1b[{total_lines}A\x1b[J");
    match io::stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            warn!("Failed to flush console {}", e);
        }
    }
    true
}
