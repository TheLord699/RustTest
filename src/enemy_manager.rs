use crate::ecs::{ECSManager, EntityID};
use crate::renderer::Renderer;
use crate::sprite::Sprite;
use crate::scripting::EnemyAI;
use serde_json::Value;

pub struct EnemyManager {
    enemy_data: Value,
    //pub ai: EnemyAI,
}

impl EnemyManager {
    pub fn new(json_path: &str) -> Self {
        let enemy_data = match std::fs::read_to_string(json_path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to parse JSON {}: {}", json_path, e);
                        Value::Object(serde_json::Map::new())
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read JSON file {}: {}", json_path, e);
                Value::Object(serde_json::Map::new())
            }
        };
        
        //let mut ai = EnemyAI::new();
        
        // Try to load AI scripts, but don't crash if it fails
        //if let Err(e) = ai.load_all_ai_scripts("assets/scripts") {
        //    eprintln!("Warning: Failed to load AI scripts: {}", e);
        //}
        
        EnemyManager { enemy_data } // EnemyManager { enemy_data, ai }
    }
    
    pub fn spawn_enemy(
        &self,
        enemy_type: &str,
        ecs_manager: &mut ECSManager,
        renderer: &mut Renderer,
        x: i32,
        y: i32,
    ) -> Option<EntityID> {
        if let Some(enemy_info) = self.enemy_data.get(enemy_type) {
            if let Some(sprite_path) = enemy_info["sprite"].as_str() {
                // Use a fallback sprite if the specified one doesn't exist
                let actual_sprite_path = if std::path::Path::new(sprite_path).exists() {
                    sprite_path
                } else {
                    "assets/sprites/missing_texture.png"
                };
                
                let scale = enemy_info["scale"].as_u64().unwrap_or(1) as u32;
                let sprite = Sprite::load(actual_sprite_path).scale(scale);
                
                let entity_id = ecs_manager.create_entity(enemy_type, x, y, 1);
                if let Some(entity) = ecs_manager.get_entity_mut(entity_id) {
                    entity.set_collider_centered(32, 32);
                    entity.simple_collisions = true;
                    entity.set_sprite(sprite.clone());
                    entity.solid = true;
                    entity.pushable = enemy_info["pushable"].as_bool().unwrap_or(true);
                    entity.mass = enemy_info["mass"].as_f64().unwrap_or(1.0) as f32;
                    
                    renderer.add_sprite_instance(entity_id, crate::renderer::SpriteInstance {
                        sprite,
                        position_x: x,
                        position_y: y,
                        z_order: 1,
                        flip_horizontal: false,
                    });
                    
                    println!("Spawned {} at ({}, {})", enemy_type, x, y);
                    return Some(entity_id);
                }
            }
        }
        
        eprintln!("Failed to spawn enemy type: {}", enemy_type);
        None
    }
    
    pub fn get_enemy_stats(&self, enemy_type: &str) -> Option<EnemyStats> {
        self.enemy_data.get(enemy_type).map(|enemy_info| {
            EnemyStats {
                health: enemy_info["health"].as_u64().unwrap_or(100) as i32,
                max_health: enemy_info["max_health"].as_u64().unwrap_or(100) as i32,
                speed: enemy_info["speed"].as_f64().unwrap_or(3.0) as f32,
                damage: enemy_info["damage"].as_u64().unwrap_or(20) as i32,
                experience: enemy_info["experience"].as_u64().unwrap_or(0) as i32,
                aggro_range: enemy_info["aggro_range"].as_u64().unwrap_or(300) as i32,
                attack_range: enemy_info["attack_range"].as_u64().unwrap_or(50) as i32,
                attack_cooldown: enemy_info["attack_cooldown"].as_f64().unwrap_or(1.5) as f32,
                knockback_resistance: enemy_info["knockback_resistance"].as_f64().unwrap_or(0.0) as f32,
                ai_type: enemy_info["ai_type"].as_str().unwrap_or("patrol").to_string(),
                pushable: enemy_info["pushable"].as_bool().unwrap_or(true),
                mass: enemy_info["mass"].as_f64().unwrap_or(1.0) as f32,
                scale: enemy_info["scale"].as_u64().unwrap_or(1) as u32,
                attack_sequence: vec![20],
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct EnemyStats {
    pub health: i32,
    pub max_health: i32,
    pub speed: f32,
    pub damage: i32,
    pub experience: i32,
    pub aggro_range: i32,
    pub attack_range: i32,
    pub attack_cooldown: f32,
    pub knockback_resistance: f32,
    pub ai_type: String,
    pub pushable: bool,
    pub mass: f32,
    pub scale: u32,
    pub attack_sequence: Vec<i32>,
}