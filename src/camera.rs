#[derive(Debug, Clone)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub viewport_width: i32,
    pub viewport_height: i32,
}

impl Camera {
    pub fn new(viewport_width: i32, viewport_height: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            viewport_width,
            viewport_height,
        }
    }

    pub fn follow(&mut self, target_x: i32, target_y: i32) {
        self.x = target_x - self.viewport_width / 2;
        self.y = target_y - self.viewport_height / 2;
    }
}
