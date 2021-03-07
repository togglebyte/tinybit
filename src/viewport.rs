use std::mem::swap;

use crate::widgets::Widget;
use crate::{Pixel, PixelBuffer, ScreenPos, ScreenSize};

/// Represents a drawable area on screen.
pub struct Viewport {
    /// The viewport's position on screen.
    /// Where 0,0 is the top left corner
    pub position: ScreenPos,

    /// The size of the viewport. Should probably match the size of the camera
    /// that is used with this viewport.
    pub size: ScreenSize,
    new_buf: PixelBuffer,
    old_buf: PixelBuffer,
}

impl Viewport {
    /// Create a new viewport with a given screen position.
    pub fn new(position: ScreenPos, size: ScreenSize) -> Self {
        Self {
            position,
            size,
            new_buf: PixelBuffer::new(size),
            old_buf: PixelBuffer::new(size),
        }
    }

    /// Resize the viewport.
    /// Remember to clear the renderer or residual
    /// characters might remain.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.size = ScreenSize::new(width, height);
        self.new_buf = PixelBuffer::new(self.size);
        self.old_buf = PixelBuffer::new(self.size);
    }

    /// Draw the pixels onto the renderable surface layers.
    /// This is offset by the camera and the viewport.
    pub fn draw_pixels(&mut self, pixels: Vec<Pixel>) {
        pixels.iter().for_each(|pixel| {
            self.draw_pixel(*pixel);
        });
    }

    /// Draw a single pixel onto the rendereable surface layers.
    /// This is called from `draw_pixels` for each pixel.
    ///
    /// This is useful if it's desired to draw just one pixel.
    pub fn draw_pixel(&mut self, pixel: Pixel) {
        if self.in_view(pixel.pos) {
            self.new_buf.set_pixel(pixel);
        }
    }

    /// Draw a widget with an offset in the viewport.
    pub fn draw_widget(&mut self, widget: &impl Widget, offset: ScreenPos) {
        widget.pixels(self.size).into_iter().for_each(|mut p| {
            p.pos.x += offset.x;
            p.pos.y += offset.y;
            self.draw_pixel(p);
        })
    }

    fn in_view(&self, pos: ScreenPos) -> bool {
        pos.x < self.size.width && pos.y < self.size.height
    }

    fn offset(&self, pos: ScreenPos) -> ScreenPos {
        ScreenPos::new(pos.x + self.position.x, pos.y + self.position.y)
    }

    pub(crate) fn pixels(&mut self) -> Vec<Pixel> {
        let mut pixels = Vec::<Pixel>::new();

        for (new, old) in self
            .new_buf
            .pixels
            .iter()
            .enumerate()
            .zip(&self.old_buf.pixels)
        {
            match (new, old) {
                ((index, Some(pixel)), _) => {
                    let pos = self.offset(self.new_buf.index_to_coords(index));
                    let mut pixel = *pixel;
                    pixel.pos = pos;
                    pixels.push(pixel);
                }
                ((index, None), Some(_)) => {
                    let pos = self.offset(self.new_buf.index_to_coords(index));
                    pixels.push(Pixel::white(' ', pos));
                }
                ((_, None), None) => {}
            }
        }

        swap(&mut self.new_buf, &mut self.old_buf);
        self.new_buf.pixels.iter_mut().for_each(|opt| {
            opt.take();
        });

        pixels
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    fn camera(viewport: &Viewport) -> Camera<camera::NoLimit> {
        let pos = WorldPos::new(30.0, 30.0);
        Camera::from_viewport(pos, viewport)
    }

    fn viewport() -> Viewport {
        let pos = ScreenPos::new(2, 2);
        let size = ScreenSize::new(6, 6);
        Viewport::new(pos, size)
    }

    #[test]
    fn draw_corners() {
        let mut view = viewport();
        let cam = camera(&view);

        let min_x = cam.bounding_box.min_x();
        let max_x = cam.bounding_box.max_x();
        let min_y = cam.bounding_box.min_y();
        let max_y = cam.bounding_box.max_y();

        let a = WorldPos::new(min_x, min_y);
        let b = WorldPos::new(max_x - 1.0, min_y);
        let c = WorldPos::new(min_x, max_y - 1.0);
        let d = WorldPos::new(max_x - 1.0, max_y - 1.0);

        let positions = vec![a, b, c, d];
        let glyphs = vec!['A', 'B', 'C', 'D'];
        let pixels = positions
            .into_iter()
            .zip(glyphs)
            .map(|(p, g)| Pixel::new(g, cam.to_screen(p), None, None))
            .collect::<Vec<_>>();

        view.draw_pixels(pixels);

        let a = Pixel::new('A', ScreenPos::new(2, 2), None, None);
        let b = Pixel::new('B', ScreenPos::new(7, 2), None, None);
        let c = Pixel::new('C', ScreenPos::new(2, 7), None, None);
        let d = Pixel::new('D', ScreenPos::new(7, 7), None, None);

        let drawn_pixels = view.pixels();

        assert_eq!(&drawn_pixels, &[a, b, c, d]);
    }
}
