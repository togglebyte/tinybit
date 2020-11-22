use crossterm::style::Color;

use crate::{Pixel, ScreenPos, ScreenSize};

pub trait Widget {
    fn pixels(&self, size: ScreenSize) -> Vec<Pixel>;
}

// -----------------------------------------------------------------------------
//     - Text -
// -----------------------------------------------------------------------------
pub struct Text(pub String, pub Option<Color>, pub Option<Color>);

impl Text {
    pub fn new(s: String, fg: Option<Color>, bg: Option<Color>) -> Self {
        Self(s, fg, bg)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Text {
        Text::new(s, None, None)
    }
}

impl Widget for Text {
    fn pixels(&self, _size: ScreenSize) -> Vec<Pixel> {
        self.0
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (y as u16, x as u16, c)))
            .map(|(y, x, c)| Pixel::new(c, ScreenPos::new(x, y), self.1, self.2))
            .collect()
    }
}

// -----------------------------------------------------------------------------
//     - Border -
// -----------------------------------------------------------------------------
pub struct Border {
    s: String,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
}

impl Border {
    /// Create a new border from the chars in `s`, starting
    /// from the top left corner, going clockwise.
    ///
    /// ```text
    /// // Border::new("ABCDEFGH" None) 
    ///
    /// ABBBBBBC
    /// H      D
    /// H      D
    /// GFFFFFFE
    /// ```
    pub fn new(s: String, fg_color: Option<Color>, bg_color: Option<Color>) -> Self {
        debug_assert!(s.chars().count() >= 8);
        Self { s, fg_color, bg_color }
    }
}

impl Widget for Border {
    fn pixels(&self, size: ScreenSize) -> Vec<Pixel> {
        let chars = self.s.chars().collect::<Vec<_>>();

        let left = chars[7];
        let bot_left = chars[6];
        let bot = chars[5];
        let bot_right = chars[4];
        let right = chars[3];
        let top_right = chars[2];
        let top = chars[1];
        let top_left = chars[0];

        let mut sides = (1..size.height - 1) // Left
            .into_iter()
            .map(|y| Pixel::new(left, ScreenPos::new(0, y), self.fg_color, self.bg_color))
            .collect::<Vec<_>>();

        sides.append(&mut (1..size.height - 1) // Right
            .into_iter()
            .map(|y| Pixel::new(right, ScreenPos::new(0 + size.width - 1, y), self.fg_color, self.bg_color))
            .collect::<Vec<_>>());

        let mut top = (1..size.width - 1)
            .into_iter()
            .map(|x| Pixel::new(top, ScreenPos::new(x, 0), self.fg_color, self.bg_color))
            .collect::<Vec<_>>();

        top.append(&mut (1..size.width - 1) // Bottom
            .into_iter()
            .map(|x| Pixel::new(bot, ScreenPos::new(x, size.height - 1), self.fg_color, self.bg_color))
            .collect::<Vec<_>>());

        top.append(&mut sides);

        // Corners
        top.push(Pixel::new(top_left, ScreenPos::zero(), self.fg_color, self.bg_color));
        top.push(Pixel::new(top_right, ScreenPos::new(size.width - 1, 0), self.fg_color, self.bg_color));
        top.push(Pixel::new(bot_right, ScreenPos::new(size.width - 1, size.height - 1), self.fg_color, self.bg_color));
        top.push(Pixel::new(bot_left, ScreenPos::new(0, size.height - 1), self.fg_color, self.bg_color));

        top
    }
}
