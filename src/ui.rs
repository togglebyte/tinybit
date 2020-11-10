use std::io::{Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::QueueableCommand;

use crate::{ScreenPos, Viewport};

/// Useful to log text to screen
pub struct DebugOutput {
    pub text: String,
}

impl DebugOutput {
    pub fn render(&self, viewport: &mut Viewport) {
        let pixels = self
            .text
            .chars()
            .enumerate()
            .map(|(index, c)| (c, ScreenPos::new(index as u16, 0)))
            .collect();
        viewport.add_to_buffer(pixels);

        // viewport.
        // let _ = stdout.queue(MoveTo(viewport.position.x, viewport.position.y));
        // let _ = stdout.queue(Print(&self.text));
        // let _ = stdout.flush();
    }
}

impl Default for DebugOutput {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}
