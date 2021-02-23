use std::io::{self, Stdout, Write};

use crossterm::cursor::{self, MoveTo};
use crossterm::style::{SetBackgroundColor, SetForegroundColor};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::QueueableCommand;
use crossterm::{execute, ExecutableCommand, Result};

use crate::{Color, Pixel, Viewport};

// -----------------------------------------------------------------------------
//     - Setup terminal for stdout target -
// -----------------------------------------------------------------------------
fn setup_terminal_for_stdout_target() -> Result<Stdout> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

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
//     - Reset terminal from stdout target -
// -----------------------------------------------------------------------------
fn reset_terminal_from_stdout_target(stdout: &mut Stdout) -> Result<()> {
    // Do we need to show the cursor too, or does that get handled
    // automatically by crossterm?

    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
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

// -----------------------------------------------------------------------------
//     - Stdout render target -
// -----------------------------------------------------------------------------
/// Render to stdout
pub struct StdoutTarget {
    stdout: Stdout,
    last_color_fg: Option<Color>,
    last_color_bg: Option<Color>,
}

impl StdoutTarget {
    /// Create a new stdout target.
    /// This sets up the terminal so tinybit can draw on it. That includes:
    /// * Enabling raw mode
    /// * Entering an alternate screen
    /// * Hiding the cursor
    /// * Clearing the screen
    ///
    /// Once this is dropped it will reset all these settings.
    pub fn new() -> Result<Self> {
        let stdout = setup_terminal_for_stdout_target()?;
        Ok(Self {
            stdout,
            last_color_fg: None,
            last_color_bg: None,
        })
    }
}

impl RenderTarget for StdoutTarget {
    fn render(&mut self, pixels: Vec<Pixel>) {
        for pixel in pixels {
            self.stdout
                .queue(MoveTo(pixel.pos.x, pixel.pos.y))
                .expect("failed to move cursor");

            // Set the foreground colour if the colour is different
            // than the last colour used
            if self.last_color_fg != pixel.fg_color {
                self.last_color_fg = pixel.fg_color;
                let _ = match self.last_color_fg {
                    Some(color) => self.stdout.queue(SetForegroundColor(color)),
                    None => self.stdout.queue(SetForegroundColor(Color::Reset)),
                };
            }

            // Set the background colour if the colour is different
            // than the last colour used
            if self.last_color_bg != pixel.bg_color {
                self.last_color_bg = pixel.bg_color;
                let _ = match self.last_color_bg {
                    Some(color) => self.stdout.queue(SetBackgroundColor(color)),
                    None => self.stdout.queue(SetBackgroundColor(Color::Reset)),
                };
            }

            self.stdout
                .queue(Print(pixel.glyph.to_string()))
                .expect("failed to print");
        }

        let _ = self.stdout.flush();
    }

    fn clear(&mut self) {
        let _ = self.stdout.execute(Clear(ClearType::All));
    }
}

impl Drop for StdoutTarget {
    fn drop(&mut self) {
        let _ = reset_terminal_from_stdout_target(&mut self.stdout);
    }
}

// -----------------------------------------------------------------------------
//     - Dummy render target -
// -----------------------------------------------------------------------------
/// A dummy render target.
pub struct DummyTarget;

impl RenderTarget for DummyTarget {
    fn render(&mut self, _pixels: Vec<Pixel>) {}
    fn clear(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    fn camera() -> Camera<camera::NoLimit> {
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
        let a = Pixel::new(a.0, cam.to_screen(a.1), None, None);

        view.draw_pixel(a);
        let mut renderer = Renderer::new(DummyTarget { pixels: Vec::new() });

        renderer.render(&mut view);

        let a = Pixel::new('A', ScreenPos::new(2, 2), None, None); // 2, 2 because of the viewport offset
        let pixels = vec![a];
        assert_eq!(pixels, renderer.target.pixels);
    }
}
