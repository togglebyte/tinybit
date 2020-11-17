use std::io::{self, Stdout, Write};

use crossterm::cursor::{self, MoveTo};
use crossterm::style::{ResetColor, SetForegroundColor};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::QueueableCommand;
use crossterm::{execute, ExecutableCommand, Result};

use crate::{Color, Pixel, Viewport};

// -----------------------------------------------------------------------------
//     - Raw mode -
// -----------------------------------------------------------------------------
fn raw_mode() -> Result<Stdout> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // we enable mouse capture because:
    // 1) DisableMouseCapture doesn't work on windows without enabling it first
    // 2) it allows to add mouse support later if needed
    //
    // ! if you want to disable mouse capture, be sure to enable it first,
    // ! or it will crash on windows.
    #[cfg(target_os = "windows")]
    execute!(stdout, EnableMouseCapture,)?;

    #[cfg(not(target_os = "windows"))]
    execute!(stdout, DisableMouseCapture,)?;

    stdout.execute(cursor::Hide)?;
    stdout.execute(Clear(ClearType::All))?;
    Ok(stdout)
}

// -----------------------------------------------------------------------------
//     - Renderer -
// -----------------------------------------------------------------------------
/// Draws characters to a render target (most likely stdout)
pub struct Renderer<T: RenderTarget> {
    pub(crate) target: T,
}

impl<T: RenderTarget> Renderer<T> {
    /// Create a new target
    pub fn new(target: T) -> Self {
        Self { target }
    }

    /// Draw characters to screen
    pub fn render(&mut self, viewport: &mut Viewport) {
        self.target.render(viewport.pixels());
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        self.target.clear();
    }
}

// -----------------------------------------------------------------------------
//     - Render target-
// -----------------------------------------------------------------------------
/// Something that a render can render to.
pub trait RenderTarget {
    fn render(&mut self, pixels: Vec<Pixel>);
    fn clear(&mut self);
}

/// Render to stdout
pub struct StdoutTarget {
    stdout: Stdout,
    last_color: Option<Color>,
}

impl StdoutTarget {
    /// Create a new stdout target.
    /// This sets stdout into raw mode.
    /// Once this is dropped it will disable raw mode.
    pub fn new() -> Result<Self> {
        let stdout = raw_mode()?;
        Ok(Self {
            stdout,
            last_color: None,
        })
    }
}

impl RenderTarget for StdoutTarget {
    fn render(&mut self, pixels: Vec<Pixel>) {
        for pixel in pixels {
            self.stdout
                .queue(MoveTo(pixel.pos.x, pixel.pos.y))
                .expect("failed to move cursor");

            if self.last_color != pixel.color {
                self.last_color = pixel.color;
                let _ = match self.last_color {
                    Some(color) => self.stdout.queue(SetForegroundColor(color)),
                    None => self.stdout.queue(ResetColor),
                };
            }

            self.stdout
                .queue(Print(pixel.glyph.to_string()))
                .expect("failed to print");
        }

        let _ = self.stdout.flush();
    }

    fn clear(&mut self) {
        let _ = self.stdout.queue(Clear(ClearType::All));
    }
}

impl Drop for StdoutTarget {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    fn camera() -> Camera {
        let pos = WorldPos::new(30.0, 30.0);
        let size = WorldSize::new(6.0, 6.0);
        Camera::new(pos, size)
    }

    fn viewport() -> Viewport {
        let pos = ScreenPos::new(2, 2);
        let size = ScreenSize::new(6, 6);
        Viewport::new(pos, size)
    }

    struct DummyTarget {
        pixels: Vec<Pixel>,
    }

    impl RenderTarget for DummyTarget {
        fn render(&mut self, pixels: Vec<Pixel>) {
            self.pixels = pixels;
        }

        fn clear(&mut self) {}
    }

    #[test]
    fn render_pixels() {
        let cam = camera();
        let mut view = viewport();

        let min_x = cam.bounding_box.min_x();
        let min_y = cam.bounding_box.min_y();

        let a = ('A', WorldPos::new(min_x, min_y));
        let a = Pixel::new(a.0, cam.to_screen(a.1), None);

        view.draw_pixel(a);
        let mut renderer = Renderer::new(DummyTarget { pixels: Vec::new() });

        renderer.render(&mut view);

        let a = Pixel::new('A', ScreenPos::new(2, 2), None); // 2, 2 because of the viewport offset
        let pixels = vec![a];
        assert_eq!(pixels, renderer.target.pixels);
    }
}
