use crate::sprite::Sprite;
use std::collections::HashMap;

pub struct Animation {
    pub frames: Vec<Sprite>,
    pub speed: f32,
    pub looped: bool,
    pub current: usize,
}

pub struct AnimationHandler {
    animations: HashMap<String, Animation>,
    pub current_state: String,
    pub frame_counter: f32,
}

impl AnimationHandler {
    pub fn new() -> Self {
        Self {
            animations: HashMap::with_capacity(16),
            current_state: String::new(),
            frame_counter: 0.0,
        }
    }
    
    pub fn add_animation(&mut self, name: &str, frames: Vec<Sprite>, speed: f32, looped: bool) {
        self.animations.insert(name.to_string(), Animation {
            frames, speed, looped, current: 0,
        });
    }
    
    pub fn set_state(&mut self, state: &str) {
        if self.current_state != state && self.animations.contains_key(state) {
            self.current_state = state.to_string();
            self.frame_counter = 0.0;
            if let Some(anim) = self.animations.get_mut(state) {
                anim.current = 0;
            }
        }
    }
    
    pub fn update(&mut self) {
        if let Some(anim) = self.animations.get_mut(&self.current_state) {
            self.frame_counter += anim.speed;
            if self.frame_counter >= 1.0 {
                self.frame_counter = 0.0;
                anim.current += 1;
                if anim.current >= anim.frames.len() {
                    anim.current = if anim.looped { 0 } else { anim.frames.len() - 1 };
                }
            }
        }
    }
    
    pub fn get_current_frame(&self) -> Option<&Sprite> {
        self.animations.get(&self.current_state).and_then(|anim| anim.frames.get(anim.current))
    }
    
    pub fn load_sprite_sheet(path: &str, frame_width: u32, frame_height: u32, frame_count: usize, scale: u32) -> Vec<Sprite> {
        let sheet = Sprite::load(path);
        let mut frames = Vec::with_capacity(frame_count);
        
        for i in 0..frame_count {
            let mut pixels = Vec::with_capacity((frame_width * frame_height) as usize);
            let start_x = i as u32 * frame_width;
            
            for y in 0..frame_height {
                let src_y = y * sheet.width();
                for x in 0..frame_width {
                    let sx = start_x + x;
                    let color = if sx < sheet.width() && y < sheet.height() {
                        sheet.pixels[(src_y + sx) as usize]
                    } else { 0 };
                    pixels.push(color);
                }
            }
            
            let mut sprite = Sprite { width: frame_width, height: frame_height, pixels };
            if scale != 1 {
                sprite = sprite.scale(scale);
            }
            frames.push(sprite);
        }
        
        frames
    }
}