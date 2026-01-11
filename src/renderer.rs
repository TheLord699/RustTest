use crate::sprite::Sprite;
use minifb::{Window, WindowOptions, Scale};
use std::collections::HashMap;

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
    sprites: HashMap<String, SpriteInstance>,
    pub camera_x: i32,
    pub camera_y: i32,
    sprite_instances: Vec<*const SpriteInstance>,
}

impl Renderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(title, width, height, WindowOptions {
            resize: true, scale: Scale::X1, ..WindowOptions::default()
        }).unwrap();
        
        Renderer {
            window,
            buffer: vec![0; width * height],
            width,
            height,
            sprites: HashMap::with_capacity(32),
            camera_x: 0,
            camera_y: 0,
            sprite_instances: Vec::with_capacity(32),
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
    
    #[inline(always)]
    fn plot_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.buffer[(y as usize) * self.width + (x as usize)] = color;
        }
    }
    
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;
        
        let mut x = x1;
        let mut y = y1;
        
        loop {
            self.plot_pixel(x, y, color);
            if x == x2 && y == y2 { break; }
            
            let e2 = err;
            if e2 > -dx { err -= dy; x += sx; }
            if e2 < dy { err += dx; y += sy; }
        }
    }
    
    pub fn draw_rectangle(&mut self, left: i32, top: i32, width: u32, height: u32, color: u32, thickness: i32) {
        let right = left + width as i32;
        let bottom = top + height as i32;
        
        for i in 0..thickness.min(width as i32 / 2).min(height as i32 / 2) {
            self.draw_line(left, top + i, right, top + i, color);
            self.draw_line(left, bottom - i - 1, right, bottom - i - 1, color);
            self.draw_line(left + i, top, left + i, bottom, color);
            self.draw_line(right - i - 1, top, right - i - 1, bottom, color);
        }
    }
    
    pub fn draw_filled_rectangle(&mut self, left: i32, top: i32, width: u32, height: u32, color: u32) {
        let left = left.max(0);
        let top = top.max(0);
        let right = (left + width as i32).min(self.width as i32 - 1);
        let bottom = (top + height as i32).min(self.height as i32 - 1);
        
        for y in top..bottom {
            let row_start = (y as usize) * self.width;
            for x in left..right {
                self.buffer[row_start + x as usize] = color;
            }
        }
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
    
    pub fn get_sprite_instance_mut(&mut self, name: &str) -> Option<&mut SpriteInstance> {
        self.sprites.get_mut(name)
    }
    
    pub fn get_sprite_instance(&self, name: &str) -> Option<&SpriteInstance> {
        self.sprites.get(name)
    }
    
    pub fn render_frame(&mut self) {
        self.buffer.fill(0x00000000);
        
        self.sprite_instances.clear();
        self.sprite_instances.extend(self.sprites.values().map(|s| s as *const _));
        self.sprite_instances.sort_by_key(|&s| unsafe { (*s).z_order });
        
        for &instance in &self.sprite_instances {
            let s = unsafe { &*instance };
            let screen_x = s.position_x - self.camera_x;
            let screen_y = s.position_y - self.camera_y;
            
            if s.flip_horizontal {
                s.sprite.draw_flipped(&mut self.buffer, self.width, self.height, screen_x, screen_y);
            } else {
                s.sprite.draw(&mut self.buffer, self.width, self.height, screen_x, screen_y);
            }
        }
        
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap_or(());
    }
    
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}