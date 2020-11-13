use crate::{Screen, ScreenPos, WorldPos, WorldRect, WorldSize, Viewport};

/// Camera
pub struct Camera {
    /// Global position
    pub position: WorldPos,

    size: WorldSize,
    pub(crate) bounding_box: WorldRect,
    limit: WorldRect,
}

impl Camera {
    /// Create a new camera at a specific world position, with a fixed
    /// size.
    // pub fn from_viewport(position: WorldPos, size: WorldSize) -> Self {
    pub fn from_viewport(position: WorldPos, viewport: &Viewport) -> Self {
        let size = WorldSize::new(viewport.size.width as isize, viewport.size.height as isize);
        Self::new(position, size)
    }

    pub fn new(position: WorldPos, size: WorldSize) -> Self {
        assert!(position.x >= size.width / 2);
        assert!(position.y >= size.height / 2);
        let bounding_box = WorldRect::new(
            WorldPos::new(position.x - size.width / 2, position.y - size.height / 2),
            size,
        );

        Self {
            position,
            size,
            bounding_box,
            limit: bounding_box,
        }
    }

    /// Convert a point to local space.
    pub fn to_screen(&self, pos: WorldPos) -> ScreenPos {
        let min_x = self.bounding_box.min_x();
        let min_y = self.bounding_box.min_y();

        ScreenPos::new(
            (pos.x - min_x) as u16,
            (pos.y - min_y) as u16,
        )
    }

    /// The limit is used for tracking. For more information see `tracking`
    pub fn set_limit(&mut self, width: u16, height: u16) {
        let width = width as isize;
        let height = height as isize;
        let origin = self.position - WorldPos::new(width / 2, height / 2);
        self.limit = WorldRect::new(origin.to_point(), WorldSize::new(width, height));
    }

    /// Move the camera to a new position in global space
    pub fn move_to(&mut self, new_pos: WorldPos) {
        self.position = new_pos;

        // Bounding box
        self.bounding_box = WorldRect::new(
            WorldPos::new(
                self.position.x.saturating_sub(self.size.width / 2),
                self.position.y.saturating_sub(self.size.height / 2),
            ),
            self.size,
        );

        // Move limit
        self.limit = WorldRect::new(
            WorldPos::new(
                self.position.x.saturating_sub(self.limit.size.width / 2),
                self.position.y.saturating_sub(self.limit.size.height / 2),
            ),
            self.limit.size,
        );
    }

    /// Move the camera if the target is outside of the camera's `limit` box
    pub fn track(&mut self, pos: WorldPos) {
        let x = if pos.x < self.limit.min_x() {
            let min = self.position.x + self.size.width;
            let val = pos
                .x
                .saturating_add(self.limit.min_x() + self.limit.size.width - 1);
            min.max(val)
        } else if pos.x > self.limit.max_x() {
            pos.x
                .saturating_add(self.limit.max_x() - self.limit.size.width - 1)
        } else {
            self.position.x
        };

        let y = if pos.y < self.limit.min_y() {
            let min = self.position.y + self.size.height;
            let val = pos
                .y
                .saturating_add(self.limit.min_y() + self.limit.size.height - 1);
            min.max(val)
        } else if pos.y > self.limit.max_y() {
            pos.y
                .saturating_add(self.limit.max_y() - self.limit.size.height - 1)
        } else {
            self.position.y
        };

        let mut new_pos = WorldPos::new(
            x.saturating_sub(self.position.x),
            y.saturating_sub(self.position.y),
        );

        if new_pos.x == 0 {
            new_pos.x = self.position.x;
        }
        if new_pos.y == 0 {
            new_pos.y = self.position.y;
        }

        self.move_to(new_pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn camera() -> Camera {
        let pos = WorldPos::new(3, 3);
        let size = WorldSize::new(6, 6);
        Camera::new(pos, size)
    }

    #[test]
    fn world_to_screen_position() {
        let cam = camera();
        let screen_pos = cam.to_screen(cam.bounding_box.origin);
        assert_eq!(screen_pos, ScreenPos::zero());
    }

    #[test]
    fn move_camera() {
        let mut cam = camera();
        let dest = WorldPos::new(100, 100);
        cam.move_to(dest);
        assert_eq!(dest, cam.position);
    }
}
