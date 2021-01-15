//! A collection of widgets.
//!
//! ```
//! use tinybit::widgets::Text;
//! let text = Text::new("Hello, World", None, None);
//! ```
use crate::{Color, Pixel, ScreenPos, ScreenSize};
use crate::events::{KeyCode, KeyEvent};

pub trait Widget {
    fn pixels(&self, size: ScreenSize) -> Vec<Pixel>;
}

// -----------------------------------------------------------------------------
//     - Text -
// -----------------------------------------------------------------------------
/// Render a text string as a specified location.
pub struct Text(pub String, pub Option<Color>, pub Option<Color>);

impl Text {
    /// Make a new text widget.
    pub fn new(s: impl Into<String>, fg: Option<Color>, bg: Option<Color>) -> Self {
        Self(s.into(), fg, bg)
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
/// Render a border.
/// See the `new` function for more details.
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
    /// // Border::new("ABCDEFGH" None, None)
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

// -----------------------------------------------------------------------------
//     - Text widget -
// -----------------------------------------------------------------------------
/// A text input field.
pub struct TextField {
    pub text: String,
    pub password: bool,
    pub focus: bool,
    pub submit: bool,
    pub enabled: bool,
    pub max_length: Option<usize>,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    cursor: usize,
}

impl TextField {
    /// Construct a new instance of an input.
    pub fn new(fg_color: Option<Color>, bg_color: Option<Color>) -> Self {
        Self {
            text: String::new(),
            password: false,
            focus: false,
            submit: false,
            enabled: true,
            max_length: None,
            fg_color,
            bg_color,
            cursor: 0,
        }
    }

    /// Clear the input and place the cursor
    /// at the start.
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }

    /// Remove focus from the input.
    /// This hides the cursor.
    pub fn unfocus(&mut self) {
        self.focus = false;
        self.cursor = self.text.chars().count();
    }

    /// Pass a `KeyEvent` to the input to build
    /// up the text value.
    ///
    /// This can be accessed through the `text` field.
    pub fn event(&mut self, event: KeyEvent) {
        if !self.focus || !self.enabled {
            return;
        }

        let KeyEvent { code, .. } = event;

        match code {
            KeyCode::Left if self.cursor > 0 => {
                self.cursor -= 1;
            }
            KeyCode::Right if self.cursor < self.text.len() => {
                self.cursor += 1;
            }
            KeyCode::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.text.remove(self.cursor);
            }
            KeyCode::Delete if self.text.len() > 0 => {
                if self.text.len() <= self.cursor {
                    return;
                }
                self.text.remove(self.cursor);
                if self.cursor > self.text.len() {
                    self.cursor = self.text.len();
                }
            }
            KeyCode::Enter => {
                self.submit = true;
            }
            KeyCode::Char(c) => {
                match self.max_length {
                    Some(max_len) if max_len <= self.text.chars().count() => return,
                    _ => {}
                }

                self.text.insert(self.cursor, c);
                self.cursor += 1;
            }
            _ => {}
        }
    }
}

impl Widget for TextField {
    fn pixels(&self, _size: ScreenSize) -> Vec<Pixel> {
        let mut pixels = self
            .text
            .chars()
            .enumerate()
            .map(|(x, c)| if self.password { (x, '*') } else { (x, c) })
            .map(|(x, c)| Pixel::new(c, ScreenPos::new(x as u16, 0), self.fg_color, self.bg_color))
            .collect::<Vec<Pixel>>();

        if !self.focus || !self.enabled {
            return pixels;
        }

        // Get char under cursor
        let c = match self.password {
            true => self
                .text
                .chars()
                .nth(self.cursor)
                .map(|_| '*')
                .unwrap_or(' '),
            false => self.text.chars().nth(self.cursor).unwrap_or(' '),
        };

        // Draw cursor
        pixels.push(Pixel::new(
            c,
            ScreenPos::new(self.cursor as u16, 0),
            Some(Color::Black),
            Some(self.fg_color.unwrap_or(Color::White)),
        ));

        pixels
    }
}
