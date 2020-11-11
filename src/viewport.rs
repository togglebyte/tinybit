use std::mem::swap;

use crate::{CharBuf, Pixel, ScreenPos, ScreenSize};

/// Represents a drawable area on screen.
pub struct Viewport {
    /// The viewport's position on screen.
    /// Where 0,0 is the top left corner
    pub position: ScreenPos,

    /// The size of the viewport. Should probably match the size of the camera
    /// that is used with this viewport.
    pub size: ScreenSize,
    new_buf: CharBuf,
    old_buf: CharBuf,
}

impl Viewport {
    /// Create a new viewport with a given screen position.
    pub fn new(position: ScreenPos, size: ScreenSize) -> Self {
        Self {
            position,
            size,
            new_buf: CharBuf::new(size),
            old_buf: CharBuf::new(size),
        }
    }

    /// Draw the pixels onto the renderable surface layers.
    /// This is offset by the camera and the viewport.
    pub fn draw_pixels(&mut self, pixels: Vec<Pixel>) {
        pixels
            .iter()
            // .filter(|ent| camera.bounding_box.contains(ent.1))
            .for_each(|pixel| {
                // let pos = camera.to_screen(ent.1);
                // let pixel = (ent.0, pos);
                self.draw_pixel(*pixel);
            });
    }

    pub fn draw_pixel(&mut self, pixel: Pixel) {
        if self.in_view(pixel.1) {
            self.new_buf.set_pixel(pixel);
        }
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
            .chars
            .iter()
            .enumerate()
            .zip(&self.old_buf.chars)
        {
            match (new, old) {
                ((index, Some(c)), _) => {
                    let pos = self.offset(self.new_buf.index_to_coords(index));
                    pixels.push((*c, pos));
                }
                ((index, None), Some(_)) => {
                    let pos = self.offset(self.new_buf.index_to_coords(index));
                    pixels.push((' ', pos));
                }
                ((_, None), None) => {}
            }
        }

        swap(&mut self.new_buf, &mut self.old_buf);
        self.new_buf.chars.iter_mut().for_each(|opt| {
            opt.take();
        });

        pixels
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    fn camera() -> Camera {
        let pos = WorldPos::new(30, 30);
        let size = WorldSize::new(6, 6);
        Camera::new(pos, size)
    }

    fn viewport() -> Viewport {
        let pos = ScreenPos::new(2, 2);
        let size = ScreenSize::new(6, 6);
        Viewport::new(pos, size)
    }

    #[test]
    fn draw_corners() {
        let cam = camera();
        let mut view = viewport();

        let min_x = cam.bounding_box.min_x();
        let max_x = cam.bounding_box.max_x();
        let min_y = cam.bounding_box.min_y();
        let max_y = cam.bounding_box.max_y();

        let a = WorldPos::new(min_x, min_y);
        let b = WorldPos::new(max_x - 1, min_y);
        let c = WorldPos::new(min_x, max_y - 1);
        let d = WorldPos::new(max_x - 1, max_y - 1);

        let positions = vec![a, b, c, d];
        let glyphs = vec!['A', 'B', 'C', 'D'];
        let pixels = positions
            .into_iter()
            .zip(glyphs)
            .map(|(p, g)| (g, cam.to_screen(p)))
            .collect::<Vec<_>>();

        view.draw_pixels(pixels);

        let a = ('A', ScreenPos::new(2, 2));
        let b = ('B', ScreenPos::new(7, 2));
        let c = ('C', ScreenPos::new(2, 7));
        let d = ('D', ScreenPos::new(7, 7));

        let drawn_pixels = view.pixels();

        assert_eq!(&drawn_pixels, &[a, b, c, d]);
    }
}
