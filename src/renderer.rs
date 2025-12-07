use crate::sprite::Sprite;
use minifb::Window;
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
}

impl Renderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(title, width, height, minifb::WindowOptions::default()).unwrap();
        let buffer = vec![0; width * height];

        Renderer {
            window,
            buffer,
            width,
            height,
            sprites: HashMap::new(),
        }
    }

    pub fn add_sprite_instance(&mut self, name: &str, sprite_instance: SpriteInstance) {
        self.sprites.insert(name.to_string(), sprite_instance);
    }

    pub fn move_sprite(&mut self, name: &str, x: i32, y: i32) {
        if let Some(sprite_instance) = self.sprites.get_mut(name) {
            sprite_instance.position_x = x;
            sprite_instance.position_y = y;
        }
    }

    pub fn render_frame(&mut self) {
        self.buffer.fill(0);

        let mut sprite_instances: Vec<&SpriteInstance> = self.sprites.values().collect();
        sprite_instances.sort_by_key(|s| s.z_order);

        for sprite_instance in sprite_instances {
            sprite_instance.sprite.draw(
                &mut self.buffer,
                self.width,
                self.height,
                sprite_instance.position_x,
                sprite_instance.position_y,
            );
        }

        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}