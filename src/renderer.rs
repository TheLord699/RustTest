use crate::sprite::Sprite;
use minifb::{Window, WindowOptions, Scale};
use std::collections::HashMap;

pub struct SpriteInstance {
    pub sprite: Sprite,
    pub position_x: i32,
    pub position_y: i32,
    pub z_order: i32,
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
            s.sprite.draw(
                &mut self.buffer,
                self.width,
                self.height,
                s.position_x - self.camera_x,
                s.position_y - self.camera_y,
            );
        }

        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
