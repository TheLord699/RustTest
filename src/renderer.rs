use crate::sprite::Sprite;
use minifb::{Window, WindowOptions, Scale};
use std::collections::HashMap;
use crate::ecs::EntityID;

pub struct SpriteInstance {
    pub sprite: Sprite,
    pub position_x: i32,
    pub position_y: i32,
    pub z_order: i32,
    pub flip_horizontal: bool,
}

pub struct Renderer {
    pub window: Window,
    buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    sprites: HashMap<EntityID, SpriteInstance>,
    pub camera_x: i32,
    pub camera_y: i32,
}

impl Renderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                scale: Scale::X1,
                ..WindowOptions::default()
            },
        ).unwrap();

        Self {
            window,
            buffer: vec![0; width * height],
            width,
            height,
            sprites: HashMap::with_capacity(64),
            camera_x: 0,
            camera_y: 0,
        }
    }

    pub fn set_camera(&mut self, x: i32, y: i32) {
        self.camera_x = x;
        self.camera_y = y;
    }

    pub fn add_sprite_instance(&mut self, id: EntityID, sprite_instance: SpriteInstance) {
        self.sprites.insert(id, sprite_instance);
    }

    pub fn move_sprite(&mut self, id: EntityID, x: i32, y: i32) {
        if let Some(s) = self.sprites.get_mut(&id) {
            s.position_x = x;
            s.position_y = y;
        }
    }

    pub fn get_sprite_instance_mut(&mut self, id: EntityID) -> Option<&mut SpriteInstance> {
        self.sprites.get_mut(&id)
    }

    pub fn draw_rectangle(&mut self, left: i32, top: i32, width: u32, height: u32, color: u32, thickness: i32) {
        let right = left + width as i32;
        let bottom = top + height as i32;

        for i in 0..thickness {
            self.draw_line(left, top + i, right, top + i, color);
            self.draw_line(left, bottom - i - 1, right, bottom - i - 1, color);
            self.draw_line(left + i, top, left + i, bottom, color);
            self.draw_line(right - i - 1, top, right - i - 1, bottom, color);
        }
    }

    pub fn draw_filled_rectangle(&mut self, left: i32, top: i32, width: u32, height: u32, color: u32) {
        let right = (left + width as i32).min(self.width as i32);
        let bottom = (top + height as i32).min(self.height as i32);

        for y in top.max(0)..bottom {
            let row = y as usize * self.width;
            for x in left.max(0)..right {
                let idx = row + x as usize;
                if idx < self.buffer.len() {
                    self.buffer[idx] = color;
                }
            }
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                self.buffer[y as usize * self.width + x as usize] = color;
            }

            if x == x2 && y == y2 {
                break;
            }

            let e2 = err * 2;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn render_frame(&mut self) {
        self.buffer.fill(0x00000000);

        let mut ordered: Vec<&SpriteInstance> = self.sprites.values().collect();
        ordered.sort_by_key(|s| s.z_order);

        for s in ordered {
            let x = s.position_x - self.camera_x;
            let y = s.position_y - self.camera_y;

            if s.flip_horizontal {
                s.sprite.draw_flipped(&mut self.buffer, self.width, self.height, x, y);
            } else {
                s.sprite.draw(&mut self.buffer, self.width, self.height, x, y);
            }
        }

        let _ = self.window.update_with_buffer(&self.buffer, self.width, self.height);
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}