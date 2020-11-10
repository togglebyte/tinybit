// #![deny(missing_docs)]
//! Terminal game engine

mod camera;
mod char_buffer;
mod render;
mod ui;
mod viewport;

pub mod events;

/// Type alias representing a pixel 
/// (a character and a position)
pub type Pixel = (char, ScreenPos);

// -----------------------------------------------------------------------------
//     - Reexports -
// -----------------------------------------------------------------------------
pub use camera::Camera;
pub use char_buffer::CharBuf;
pub use crossterm::terminal::size as term_size;
pub use render::{Renderer, StdoutTarget};
pub use ui::DebugOutput;
pub use viewport::Viewport;

// -----------------------------------------------------------------------------
//     - Euclid -
// -----------------------------------------------------------------------------
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
