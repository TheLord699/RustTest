use crate::ecs::ECSManager;
use crate::renderer::Renderer;
use crate::sprite::Sprite;
use crate::ecs::Entity;
use serde_json::Value;
use crate::helper_methods::load_json;

pub struct EnemyManager {
    enemy_data: Value,
}

impl EnemyManager {
    pub fn new(json_path: &str) -> Self {
        let enemy_data = load_json(json_path);
        EnemyManager { enemy_data }
    }

    pub fn spawn_enemy(
        &self,
        enemy_type: &str,
        ecs_manager: &mut ECSManager,
        renderer: &mut Renderer,
        x: i32,
        y: i32,
    ) {
        if let Some(enemy_info) = self.enemy_data.get(enemy_type) {
            if let Some(sprite_path) = enemy_info["sprite"].as_str() {
                let scale = enemy_info["scale"].as_f64().unwrap_or(1.0) as u32;
                let hitbox_scale = enemy_info["hitbox_scale"].as_f64().unwrap_or(1.0) as u32;
                let sprite = Sprite::load(sprite_path).scale(scale);

                let mut entity = Entity::new(enemy_type, x, y, 1).with_collider(None, true);

                if let Some(dimensions) = enemy_info["dimensions"].as_array() {
                    if dimensions.len() == 2 {
                        let w = (sprite.width as f32 * dimensions[0].as_f64().unwrap_or(1.0) as f32) as u32;
                        let h = (sprite.height as f32 * dimensions[1].as_f64().unwrap_or(1.0) as f32) as u32;

                        entity.set_collider_centered(w / hitbox_scale, h / hitbox_scale);
                    }
                }

                entity.set_sprite(sprite.clone());
                ecs_manager.add_entity(entity);

                let flip_horizontal = enemy_info["flip_horizontal"].as_bool().unwrap_or(false);

                renderer.add_sprite_instance(
                    enemy_type,
                    crate::renderer::SpriteInstance {
                        sprite,
                        position_x: x,
                        position_y: y,
                        z_order: 1,
                        flip_horizontal,
                    },
                );
            }
        }
    }
}