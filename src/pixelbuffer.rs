use crate::{ScreenPos, ScreenSize, Pixel};

/// Character buffer holds all the "pixels" to be drawn on the screen
#[derive(Debug)]
pub struct PixelBuffer {
    /// Pixels
    pub pixels: Vec<Option<Pixel>>,
    size: ScreenSize,
}

impl PixelBuffer {
    /// Create a new character buffer.
    /// Should be at least the same size as the viewport it's relative to
    pub fn new(size: ScreenSize) -> Self {
        let cap = (size.width * size.height) as usize;
        let pixels = vec![None; cap];
        Self {
            pixels,
            size,
        }
    }

    /// Convert index to x y coordinates
    pub fn index_to_coords(&self, index: usize) -> ScreenPos {
        let x = index % self.size.width as usize;
        let y = index / self.size.width as usize;
        ScreenPos::new(x as u16, y as u16)
    }

    /// Get the pixel at the given screen position
    pub fn get_pixel(&self, pos: ScreenPos) -> Option<Pixel> {
        let index = (self.size.width * pos.y + pos.x) as usize;
        match self.pixels.get(index) {
            Some(c) => *c,
            None => None,
        }
    }

    pub(crate) fn set_pixel(&mut self, pixel: Pixel) {
        let index = (self.size.width * pixel.pos.y + pixel.pos.x) as usize;
        if let Some(Some(existing_pixel)) = self.pixels.get(index) {
            if existing_pixel.glyph == pixel.glyph {
                return;
            }
        }
        self.pixels[index] = Some(pixel);
    }
}
