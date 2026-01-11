use crate::ecs::{ECSManager, Entity, EntityID};
use crate::renderer::Renderer;
use crate::animation_manager::AnimationHandler;
use minifb::Key;

pub struct Player {
    pub entity_id: EntityID,
    pub entity: Entity,
    pub movement_speed: f32,
    pub input_dx: f32,
    pub input_dy: f32,
    pub animations: AnimationHandler,
    pub facing: String,
}

impl Player {
    pub fn new(renderer: &mut Renderer) -> Self {
        let entity_id = EntityID(0);
        let mut entity = Entity::new(entity_id, "player", 100, 100, 2)
            .with_mass(2.0);
        
        let mut animations = AnimationHandler::new();
        let states = [
            ("idle", 6, 0.15),
            ("walking", 8, 0.2),
            ("jump", 3, 0.15),
            ("hurt", 4, 0.1),
            ("death", 4, 0.15),
        ];
        
        for &(state, frames, speed) in &states {
            let frame_sprites = AnimationHandler::load_sprite_sheet(
                &format!("assets/sprites/player/{}_animation.png", state),
                100, 100, frames, 2,
            );
            animations.add_animation(state, frame_sprites, speed, true);
        }
        
        animations.set_state("idle");
        
        let sprite = animations.get_current_frame().unwrap();
        let collider_width = sprite.width() / 3;
        let collider_height = sprite.height() / 3;
        entity.set_collider_centered(collider_width, collider_height);
        
        renderer.add_sprite_instance(entity_id, crate::renderer::SpriteInstance {
            sprite: sprite.clone(),
            position_x: entity.position_x,
            position_y: entity.position_y,
            z_order: entity.z_order,
            flip_horizontal: false,
        });
        
        Player {
            entity_id,
            entity,
            movement_speed: 5.0,
            input_dx: 0.0,
            input_dy: 0.0,
            animations,
            facing: "right".to_string(),
        }
    }
    
    #[inline]
    fn process_input(&mut self, renderer: &Renderer) {
        self.input_dx = 0.0;
        self.input_dy = 0.0;
        
        if renderer.window.is_key_down(Key::W) { self.input_dy -= 1.0; }
        if renderer.window.is_key_down(Key::S) { self.input_dy += 1.0; }
        if renderer.window.is_key_down(Key::A) { 
            self.input_dx -= 1.0; 
            self.facing = "left".to_string();
        }
        if renderer.window.is_key_down(Key::D) { 
            self.input_dx += 1.0; 
            self.facing = "right".to_string();
        }
        
        if self.input_dx != 0.0 || self.input_dy != 0.0 {
            let length = (self.input_dx * self.input_dx + self.input_dy * self.input_dy).sqrt();
            self.input_dx = self.input_dx / length * self.movement_speed;
            self.input_dy = self.input_dy / length * self.movement_speed;
        }
    }
    
    fn process_movement(&mut self, ecs_manager: &mut ECSManager) {
        if ecs_manager.move_entity(self.entity_id, self.input_dx, self.input_dy) {
            // Update local entity copy from ECS
            if let Some(updated) = ecs_manager.get_entity(self.entity_id) {
                self.entity.position_x = updated.position_x;
                self.entity.position_y = updated.position_y;
            }
        }
    }
    
    fn draw_hitbox_outline(&self, renderer: &mut Renderer) {
        if let Some((left, top, right, bottom)) = self.entity.get_collider_bounds() {
            let width = (right - left) as u32;
            let height = (bottom - top) as u32;
            let screen_left = left - renderer.camera_x;
            let screen_top = top - renderer.camera_y;
            renderer.draw_rectangle(screen_left, screen_top, width, height, 0xFF00FF00, 3);
            renderer.draw_filled_rectangle(screen_left, screen_top, width, height, 0x2200FF00);
        }
    }
    
    fn update_renderer(&mut self, renderer: &mut Renderer) {
        self.animations.update();
        
        if let Some(frame) = self.animations.get_current_frame() {
            self.entity.set_sprite(frame.clone());
            
            renderer.move_sprite(self.entity_id, self.entity.position_x, self.entity.position_y);
            
            if let Some(instance) = renderer.get_sprite_instance_mut(self.entity_id) {
                instance.sprite = frame.clone();
                instance.flip_horizontal = self.facing == "left";
            }
        }
        
        self.draw_hitbox_outline(renderer);
    }
    
    pub fn update(&mut self, renderer: &mut Renderer, ecs_manager: &mut ECSManager) {
        self.process_input(renderer);
        self.process_movement(ecs_manager);
        
        if self.input_dx.abs() > 0.0 || self.input_dy.abs() > 0.0 {
            self.animations.set_state("walking");
        } else {
            self.animations.set_state("idle");
        }
        
        self.update_renderer(renderer);
    }
}