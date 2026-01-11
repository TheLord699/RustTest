use crate::ecs::{ECSManager, EntityID};
use crate::player::Player;
use crate::renderer::Renderer;
use crate::enemy_manager::EnemyManager;
use crate::camera::Camera;
use std::time::{Duration, Instant};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct Game {
    pub player: Player,
    pub ecs_manager: ECSManager,
    enemy_manager: EnemyManager,
    pub camera: Camera,
    enemy_ids: Vec<EntityID>,
    enemy_types: Vec<String>,
    #[allow(unused)]
    stream: OutputStream,
    #[allow(unused)]
    sink: Sink,
    last_frame: Instant,
    accumulated_time: f32,
}

impl Game {
    const FPS: u64 = 60;
    const FIXED_DELTA: f32 = 1.0 / 60.0;
    
    pub fn new(renderer: &mut Renderer, width: i32, height: i32) -> Self {
        let mut ecs_manager = ECSManager::new(width, height);
        let enemy_manager = EnemyManager::new("assets/settings/enemies.json");
        
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        let test_sound = Decoder::new(BufReader::new(File::open("assets/audio/music/aphex.mp3").unwrap())).unwrap();
        sink.append(test_sound);
        
        let player = Player::new(renderer);
        ecs_manager.add_entity(player.entity.clone());
        
        let mut enemy_ids = Vec::new();
        let mut enemy_types = Vec::new();
        
        let positions = [(400, 300), (500, 250), (600, 350)];
        for &(x, y) in &positions {
            if let Some(id) = enemy_manager.spawn_enemy("Bug", &mut ecs_manager, renderer, x, y) {
                enemy_ids.push(id);
                enemy_types.push("Bug".to_string());
            }
        }
        
        let mut camera = Camera::new(width, height);
        camera.follow(player.entity.position_x, player.entity.position_y);
        
        let game = Game {
            player,
            ecs_manager,
            enemy_manager,
            camera,
            enemy_ids,
            enemy_types,
            stream,
            sink,
            last_frame: Instant::now(),
            accumulated_time: 0.0,
        };
        
        renderer.set_camera(game.camera.x, game.camera.y);
        game
    }
    
    pub fn update(&mut self, renderer: &mut Renderer, delta: f32) {
        self.accumulated_time += delta;
        
        // Fixed timestep for physics
        while self.accumulated_time >= Self::FIXED_DELTA {
            self.fixed_update(renderer);
            self.accumulated_time -= Self::FIXED_DELTA;
        }
        
        // Smooth camera follow
        self.camera.follow_smooth(
            self.player.entity.position_x,
            self.player.entity.position_y,
            0.1
        );
        renderer.set_camera(self.camera.x, self.camera.y);
    }
    
    fn fixed_update(&mut self, renderer: &mut Renderer) {
        // Update player first
        self.player.update(renderer, &mut self.ecs_manager);
        
        let player_pos = (self.player.entity.position_x, self.player.entity.position_y);
        
        // Collect AI decisions first to avoid borrowing conflicts
        let mut enemy_movements = Vec::new();
        
        for (i, &enemy_id) in self.enemy_ids.iter().enumerate() {
            // Get enemy data without holding mutable reference for too long
            let enemy_type = self.enemy_types[i].clone();
            let stats = self.enemy_manager.get_enemy_stats(&enemy_type);
            
            // Get enemy position and ID
            let (enemy_x, enemy_y, enemy_vx, enemy_vy) = {
                if let Some(enemy) = self.ecs_manager.get_entity(enemy_id) {
                    (enemy.position_x, enemy.position_y, enemy.velocity_x, enemy.velocity_y)
                } else {
                    continue;
                }
            };
            
            // Create a temporary enemy for AI calculations
            let mut temp_enemy = crate::ecs::Entity::new(
                enemy_id,
                &enemy_type,
                enemy_x,
                enemy_y,
                1
            );
            temp_enemy.velocity_x = enemy_vx;
            temp_enemy.velocity_y = enemy_vy;
            
            // Create a temporary player entity for AI calculations
            let temp_player = crate::ecs::Entity::new(
                crate::ecs::EntityID(0),
                "player",
                player_pos.0,
                player_pos.1,
                0
            );
            
            // Calculate AI velocity
            match self.enemy_manager.ai.update_enemy(
                &enemy_type,
                enemy_id,
                &mut temp_enemy,
                &temp_player,
                stats,
            ) {
                Ok((ai_vx, ai_vy)) => {
                    // Store movement for this enemy
                    enemy_movements.push((enemy_id, ai_vx * Self::FIXED_DELTA, ai_vy * Self::FIXED_DELTA));
                }
                Err(e) => {
                    eprintln!("AI error for enemy {}: {}", enemy_type, e);
                }
            }
        }
        
        // Apply movements one at a time to prevent chain reactions
        for (enemy_id, move_x, move_y) in enemy_movements {
            // Temporarily mark other enemies as non-solid to prevent them from affecting each other
            let mut original_solid_states = Vec::new();
            
            // Use the public iterator method instead of accessing private field
            for other_id in self.ecs_manager.iter_entity_ids() {
                if other_id != enemy_id && other_id != self.player.entity_id {
                    if let Some(other_entity) = self.ecs_manager.get_entity(other_id) {
                        if other_entity.solid {
                            original_solid_states.push(other_id);
                        }
                    }
                }
            }
            
            // Temporarily make other enemies non-solid
            for &id in &original_solid_states {
                if let Some(entity) = self.ecs_manager.get_entity_mut(id) {
                    entity.solid = false;
                }
            }
            
            // Get old position before moving
            let old_position = self.ecs_manager.get_entity(enemy_id)
                .map(|e| (e.position_x, e.position_y));
            
            // Apply movement (enemies won't affect each other now)
            let moved = self.ecs_manager.move_entity(enemy_id, move_x, move_y);
            
            // Restore solid states
            for &id in &original_solid_states {
                if let Some(entity) = self.ecs_manager.get_entity_mut(id) {
                    entity.solid = true;
                }
            }
            
            if moved {
                // Update renderer if position changed
                if let Some((old_x, old_y)) = old_position {
                    if let Some(updated) = self.ecs_manager.get_entity(enemy_id) {
                        if updated.position_x != old_x || updated.position_y != old_y {
                            renderer.move_sprite(enemy_id, updated.position_x, updated.position_y);
                        }
                    }
                }
            }
        }
    }
    
    pub fn run(&mut self, renderer: &mut Renderer) {
        let frame_duration = Duration::from_millis(1000 / Self::FPS);
        
        while renderer.is_open() {
            let start = Instant::now();
            
            let delta = self.last_frame.elapsed().as_secs_f32();
            self.last_frame = Instant::now();
            
            self.update(renderer, delta);
            renderer.render_frame();
            
            let elapsed = start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }
    }
}