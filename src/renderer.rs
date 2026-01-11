use crate::sprite::Sprite;
use minifb::{Window, WindowOptions, Scale};
use std::collections::HashMap;

pub struct SpriteInstance {
    pub sprite: Sprite,
    pub position_x: i32,  // This should be CENTER position
    pub position_y: i32,  // This should be CENTER position
    pub z_order: i32,
    pub flip_horizontal: bool,
}

pub struct Renderer {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub sprites: HashMap<String, SpriteInstance>,
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

        Renderer {
            window,
            buffer: vec![0; width * height],
            width,
            height,
            sprites: HashMap::new(),
            camera_x: 0,
            camera_y: 0,
        }
    }

    pub fn set_camera(&mut self, x: i32, y: i32) {
        self.camera_x = x;
        self.camera_y = y;
    }

    pub fn set_camera_smooth(&mut self, x: i32, y: i32, modifier: f32) {
        let dx = x - self.camera_x;
        let dy = y - self.camera_y;

        self.camera_x += (dx as f32 * modifier) as i32;
        self.camera_y += (dy as f32 * modifier) as i32;
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;
        
        let mut x = x1;
        let mut y = y1;
        
        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let index = (y as usize) * self.width + (x as usize);
                self.buffer[index] = color;
            }
            
            if x == x2 && y == y2 { break; }
            
            err2 = 2 * err;
            if err2 > -dx {
                err -= dy;
                x += sx;
            }
            if err2 < dy {
                err += dx;
                y += sy;
            }
        }
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
        for dy in 0..height as i32 {
            let py = top + dy;
            if py < 0 || py >= self.height as i32 { continue; }

            for dx in 0..width as i32 {
                let px = left + dx;
                if px < 0 || px >= self.width as i32 { continue; }

                let index = (py as usize) * self.width + (px as usize);
                self.buffer[index] = color;
            }
        }
    }

    pub fn add_debug_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: u32) {
        self.draw_rectangle(x, y, width, height, color, 1);
    }

    pub fn add_sprite_instance(&mut self, name: &str, sprite_instance: SpriteInstance) {
        self.sprites.insert(name.to_string(), sprite_instance);
    }

    pub fn move_sprite(&mut self, name: &str, x: i32, y: i32) {
        if let Some(s) = self.sprites.get_mut(name) {
            s.position_x = x;
            s.position_y = y;
        }
    }

    pub fn render_frame(&mut self) {
        self.buffer.fill(0x00000000);

        let mut sprites: Vec<&SpriteInstance> = self.sprites.values().collect();
        sprites.sort_by_key(|s| s.z_order);

        for s in sprites {
            let screen_x = s.position_x - self.camera_x;
            let screen_y = s.position_y - self.camera_y;
            
            if s.flip_horizontal {
                s.sprite.draw_flipped(
                    &mut self.buffer,
                    self.width,
                    self.height,
                    screen_x,
                    screen_y,
                );
            } else {
                s.sprite.draw(
                    &mut self.buffer,
                    self.width,
                    self.height,
                    screen_x,
                    screen_y,
                );
            }
        }

        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}