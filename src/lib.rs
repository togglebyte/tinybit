// #![deny(missing_docs)]
//! Terminal game engine
//! ```
//! # use tinybit::events::{events, Event, KeyCode, KeyEvent, EventModel};
//! # use tinybit::{
//! #     term_size, Camera, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport,
//! #     WorldPos, WorldSize, Pixel
//! # };
//! 
//! fn main() {
//!     let (width, height) = term_size().expect("Can't get the term size? Can't play the game!");
//! 
//!     // Viewport
//!     let viewport_size = ScreenSize::new(width / 2, height / 2);
//!     let mut viewport = Viewport::new(ScreenPos::new(0, 4), viewport_size);
//! 
//!     // Camera
//!     let (width, height) = (width as f32, height as f32);
//!     let camera_size = WorldSize::new(width / 2.0, height / 2.0); let camera_pos =
//!     WorldPos::new(width, height);
//!     let mut camera = Camera::new(camera_pos, camera_size);
//! 
//!     // Renderer
//!     let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
//!     let mut renderer = Renderer::new(stdout_renderer);
//! 
//!     // Player
//!     let mut player = ('@', camera_pos);
//! 
//!     for event in events(EventModel::Fps(20)) {
//!         match event {
//!             Event::Tick => {
//!                 let pixel = Pixel::new(player.0, camera.to_screen(player.1), None, None);
//!                 viewport.draw_pixel(pixel);
//!                 let _ = renderer.render(&mut viewport);
//! #               break
//!             }
//!             Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
//!             Event::Key(KeyEvent { code: kc, .. }) => {
//!                 match kc {
//!                     KeyCode::Left => { player.1.x -= 1.0; }
//!                     KeyCode::Right => { player.1.x += 1.0; }
//!                     KeyCode::Up => { player.1.y -= 1.0; }
//!                     KeyCode::Down => { player.1.y += 1.0; }
//!                     _ => {}
//!                 }
//!             }
//!             Event::Resize(w, h) => {}
//!         }
//!     }
//! }
//! ```

use serde::{Serialize, Deserialize};

mod camera;
mod pixelbuffer;
mod render;
mod viewport;

pub mod events;
pub mod widgets;

/// A character at a position, with a colour
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub glyph: char,
    pub pos: ScreenPos,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Pixel {
    pub fn new(glyph: char, pos: ScreenPos, fg_color: Option<Color>, bg_color: Option<Color>) -> Self {
        Self {
            glyph,
            pos,
            fg_color,
            bg_color,
        }
    } 

    pub fn white(c: char, pos: ScreenPos) -> Self {
        Self::new(c, pos, None, None)
    }
}

// -----------------------------------------------------------------------------
//     - Reexports -
// -----------------------------------------------------------------------------
pub use camera::Camera;
pub use pixelbuffer::PixelBuffer;
pub use crossterm::terminal::size as term_size;
pub use render::{Renderer, StdoutTarget};
pub use viewport::Viewport;
pub use crossterm::style::{Colored, Color};

// -----------------------------------------------------------------------------
//     - Euclid -
// -----------------------------------------------------------------------------
pub type Vec2D<T> = euclid::default::Vector2D<T>;

/// Constraining units to screen space
#[derive(Serialize, Deserialize, Debug)]
pub struct Screen;

/// Constraining units to world space
#[derive(Serialize, Deserialize, Debug)]
pub struct World;

/// A position on screen, where 0,0 is the top left corner
pub type ScreenPos = euclid::Point2D<u16, Screen>;

/// A position in the world
pub type WorldPos = euclid::Point2D<f32, World>;

/// A rect on screen
pub type ScreenRect = euclid::Rect<u16, Screen>;

/// A rect in the world
pub type WorldRect = euclid::Rect<f32, World>;

/// A size on screen
pub type ScreenSize = euclid::Size2D<u16, Screen>;

/// A size in the world
pub type WorldSize = euclid::Size2D<f32, World>;
