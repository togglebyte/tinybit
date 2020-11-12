use crossterm::style::Color;

use crate::{Pixel, ScreenPos, ScreenSize};

pub trait Widget {
    fn pixels(&self, offset: ScreenPos, size: ScreenSize) -> Vec<Pixel>;
}

// -----------------------------------------------------------------------------
//     - Text -
// -----------------------------------------------------------------------------
pub struct Text(pub String, pub Option<Color>);

impl Text {
    pub fn new(s: String, col: Option<Color>) -> Self {
        Self(s, col)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Text {
        Text::new(s, None)
    }
}

impl Widget for Text {
    fn pixels(&self, offset: ScreenPos, size: ScreenSize) -> Vec<Pixel> {
        let mut y = 0;
        self.0
            .chars()
            .enumerate()
            .map(|(x, c)| Pixel::new(c, ScreenPos::new(x as u16, y), self.1))
            .collect()
    }
}

// -----------------------------------------------------------------------------
//     - Border -
// -----------------------------------------------------------------------------
pub struct Border {
    s: String,
    size: ScreenSize,
    color: Option<Color>,
}

impl Border {
    pub fn new(s: String, size: ScreenSize, color: Option<Color>) -> Self {
        debug_assert!(s.chars().count() >= 8);
        debug_assert!(size.width > 2);
        debug_assert!(size.height > 2);
        Self { s, size, color }
    }
}

impl Widget for Border {
    fn pixels(&self, offset: ScreenPos, size: ScreenSize) -> Vec<Pixel> {
        let chars = self.s.chars().collect::<Vec<_>>();

        let left = chars[7];
        let bot_left = chars[6];
        let bot = chars[5];
        let bot_right = chars[4];
        let right = chars[3];
        let top_right = chars[2];
        let top = chars[1];
        let top_left = chars[0];

        // let mut pixels = Vec::with_capacity(size.width as usize * 2 + size.height as usize * 2);

        let mut sides = (1..size.height - 1) // Left
            .into_iter()
            .map(|y| Pixel::new(left, ScreenPos::new(offset.x, y + offset.y), self.color))
            .collect::<Vec<_>>();

        sides.append(&mut (1..size.height - 1) // Right
            .into_iter()
            .map(|y| Pixel::new(right, ScreenPos::new(offset.x + size.width - 1, offset.y + y), self.color))
            .collect::<Vec<_>>());

        let mut top = (1..size.width - 1)
            .into_iter()
            .map(|x| Pixel::new(top, ScreenPos::new(offset.x + x, offset.y), self.color))
            .collect::<Vec<_>>();

        top.append(&mut (1..size.width - 1) // Bottom
            .into_iter()
            .map(|x| Pixel::new(bot, ScreenPos::new(offset.x + x, offset.y + size.height - 1), self.color))
            .collect::<Vec<_>>());

        top.append(&mut sides);

        // Corners
        top.push(Pixel::new(top_left, offset, self.color));
        top.push(Pixel::new(top_right, ScreenPos::new(offset.x + size.width - 1, offset.y), self.color));
        top.push(Pixel::new(bot_right, ScreenPos::new(offset.x + size.width - 1, offset.y + size.height - 1), self.color));
        top.push(Pixel::new(bot_left, ScreenPos::new(offset.x, offset.y + size.height - 1), self.color));

        top
    }
}
