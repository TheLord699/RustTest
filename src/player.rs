use crate::ecs::{ECSManager, Entity, Collider};
use crate::renderer::Renderer;
use crate::sprite::Sprite;
use minifb::Key;

pub struct Player {
    pub entity: Entity,
    pub movement_speed: f32,
    pub input_dx: f32,
    pub input_dy: f32,
}

impl Player {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut entity = Entity::new("player", 100, 100, 2);
        let sprite = Sprite::load("assets/sprites/bug.png").scale(3);

        let collider = Collider {width: (sprite.width * 2 / 3) as u32, height: (sprite.height * 2 / 3) as u32, offset_x: 0,offset_y: 0,};

        // let mut entity = Entity::new("player", 100, 100, 2).with_collider(Some(collider), false); with collider still not setting centre correctly
        
        let collider_width = (sprite.width * 2 / 3) as u32;
        let collider_height = (sprite.height * 2 / 3) as u32;
        entity.set_collider_centered(collider_width, collider_height);
        entity.set_sprite(sprite.clone());

        renderer.add_sprite_instance(
            "player",
            crate::renderer::SpriteInstance {
                sprite,
                position_x: entity.position_x,
                position_y: entity.position_y,
                z_order: entity.z_order,
            },
        );

        Player {
            entity,
            movement_speed: 5.0,
            input_dx: 0.0,
            input_dy: 0.0,
        }
    }

    fn process_input(&mut self, renderer: &Renderer) {
        self.capture_keyboard_input(renderer);
        self.normalize_movement();
    }   

    /*
    fn handle_collisions(&mut self, ecs_manager: &ECSManager) {
        if let Some(player_idx) = self.find_player_in_ecs(ecs_manager) {
            if let Some(collisions) = ecs_manager.get_collisions(player_idx) {
                for collision in collisions {
                    if collision.name == "test"{
                        return;
                    }
                }
            }
        }
    }
    */

    fn capture_keyboard_input(&mut self, renderer: &Renderer) {
        self.input_dx = 0.0;
        self.input_dy = 0.0;

        if renderer.window.is_key_down(Key::W) { self.input_dy -= 1.0; }
        if renderer.window.is_key_down(Key::S) { self.input_dy += 1.0; }
        if renderer.window.is_key_down(Key::A) { self.input_dx -= 1.0; }
        if renderer.window.is_key_down(Key::D) { self.input_dx += 1.0; }
    }

    fn normalize_movement(&mut self) {
        if self.input_dx != 0.0 || self.input_dy != 0.0 {
            let length = (self.input_dx * self.input_dx + self.input_dy * self.input_dy).sqrt();
            self.input_dx = self.input_dx / length * self.movement_speed;
            self.input_dy = self.input_dy / length * self.movement_speed;
        }
    }

    fn process_movement(&mut self, ecs_manager: &mut ECSManager) {
        if let Some(player_idx) = self.find_player_in_ecs(ecs_manager) {
            let moved = self.attempt_movement_in_ecs(ecs_manager, player_idx);
            if moved {
                self.sync_with_ecs(ecs_manager, player_idx);
            }
        }
        
        self.reset_movement();
    }

    fn find_player_in_ecs(&self, ecs_manager: &ECSManager) -> Option<usize> {
        ecs_manager.get_entity_by_name(&self.entity.name)
    }

    fn attempt_movement_in_ecs(&self, ecs_manager: &mut ECSManager, player_idx: usize) -> bool {
        ecs_manager.move_entity(player_idx, self.input_dx, self.input_dy)
    }

    fn sync_with_ecs(&mut self, ecs_manager: &ECSManager, player_idx: usize) {
        if let Some(updated_entity) = ecs_manager.get_entity(player_idx) {
            self.entity.position_x = updated_entity.position_x;
            self.entity.position_y = updated_entity.position_y;
        }
    }

    fn reset_movement(&mut self) {
        self.input_dx = 0.0;
        self.input_dy = 0.0;
    }

    fn update_renderer(&mut self, renderer: &mut Renderer) {
        renderer.move_sprite(
            &self.entity.name,
            self.entity.position_x,
            self.entity.position_y
        );
    }

    pub fn update(&mut self, renderer: &mut Renderer, ecs_manager: &mut ECSManager) {
        self.process_input(renderer);
        self.process_movement(ecs_manager);
        self.update_renderer(renderer);
    }
}