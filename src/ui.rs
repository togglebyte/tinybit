use crate::{ScreenPos, Viewport};

/// Useful to log text to screen
pub struct DebugOutput {
    pub text: String,
}

impl DebugOutput {
    pub fn render(&self, viewport: &mut Viewport) {
        self
            .text
            .chars()
            .enumerate()
            .for_each(|(index, c)| {
                let pixel = (c, ScreenPos::new(index as u16, 0));
                viewport.draw_pixel(pixel);
            });
    }
}

impl Default for DebugOutput {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}
