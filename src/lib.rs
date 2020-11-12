// #![deny(missing_docs)]
//! Terminal game engine
//! ```
//! # use tinybit::events::{events, Event, KeyCode, KeyEvent};
//! # use tinybit::{
//! #     term_size, Camera, DebugOutput, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport,
//! #     WorldPos, WorldSize,
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
//!     let camera_size = WorldSize::new(width / 2, height / 2); let camera_pos =
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
//!     for event in events(20) {
//!         match event {
//!             Event::Tick => {
//!                 let pixel = (player.0, camera.to_screen(player.1));
//!                 viewport.draw_pixel(pixel);
//!                 let _ = renderer.render(&mut viewport);
//! #               break
//!             }
//!             Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
//!             Event::Key(KeyEvent { code: kc, .. }) => {
//!                 match kc {
//!                     KeyCode::Left => { player.1.x -= 1; }
//!                     KeyCode::Right => { player.1.x += 1; }
//!                     KeyCode::Up => { player.1.y -= 1; }
//!                     KeyCode::Down => { player.1.y += 1; }
//!                     _ => {}
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

mod camera;
mod pixelbuffer;
mod render;
mod ui;
mod viewport;

pub mod events;
pub mod widgets;

/// A character at a position, with a colour
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    glyph: char,
    pos: ScreenPos,
    color: Option<Color>,
}

impl Pixel {
    pub fn new(glyph: char, pos: ScreenPos, color: Option<Color>) -> Self {
        Self {
            glyph,
            pos,
            color,
        }
    } 

    pub fn white(c: char, pos: ScreenPos) -> Self {
        Self::new(c, pos, None)
    }
}

// -----------------------------------------------------------------------------
//     - Reexports -
// -----------------------------------------------------------------------------
pub use camera::Camera;
pub use pixelbuffer::PixelBuffer;
pub use crossterm::terminal::size as term_size;
pub use render::{Renderer, StdoutTarget};
pub use ui::DebugOutput;
pub use viewport::Viewport;
pub use crossterm::style::Color;

// -----------------------------------------------------------------------------
//     - Euclid -
// -----------------------------------------------------------------------------
pub type Vec2D<T> = euclid::default::Vector2D<T>;

/// Constraining units to screen space
pub struct Screen;

/// Constraining units to world space
pub struct World;

/// A position on screen, where 0,0 is the top left corner
pub type ScreenPos = euclid::Point2D<u16, Screen>;

/// A position in the world
pub type WorldPos = euclid::Point2D<u16, World>;

/// A rect on screen
pub type ScreenRect = euclid::Rect<u16, Screen>;

/// A rect in the world
pub type WorldRect = euclid::Rect<u16, World>;

/// A size on screen
pub type ScreenSize = euclid::Size2D<u16, Screen>;

/// A size in the world
pub type WorldSize = euclid::Size2D<u16, World>;
