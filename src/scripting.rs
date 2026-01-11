use mlua::{Lua, Result, Table, Function};
use crate::ecs::Entity;
use crate::enemy_manager::EnemyStats;
use std::collections::HashMap;
use std::time::Instant;

pub struct EnemyAI {
    lua: Lua,
    ai_tables: HashMap<String, Function<'static>>,
    last_update: HashMap<usize, Instant>,
}

impl EnemyAI {
    pub fn new() -> Self {
        let lua = Lua::new();
        
        let _ = lua.load(r#"
                package.path = package.path .. ";assets/scripts/?.lua"
                package.path = package.path .. ";assets/scripts/presets/?.lua"
                package.path = package.path .. ";assets/scripts/enemies/?.lua"
                package.path = package.path .. ";./assets/scripts/?.lua"
                package.path = package.path .. ";./assets/scripts/presets/?.lua"
                package.path = package.path .. ";./assets/scripts/enemies/?.lua"
            "#).exec();
        
        Self {
            lua,
            ai_tables: HashMap::new(),
            last_update: HashMap::new(),
        }
    }

    pub fn load_enemy_ai(&mut self, enemy_type: &str, path: &str) -> Result<()> {
        let code = match std::fs::read_to_string(path) {
            Ok(code) => code,
            Err(e) => {
                let alt_path = format!("assets/scripts/enemies/{}.lua", enemy_type.to_lowercase());
                match std::fs::read_to_string(&alt_path) {
                    Ok(code) => code,
                    Err(_) => {
                        return Err(mlua::Error::RuntimeError(format!(
                            "Failed to load AI script for {}: {} (tried {} and {})",
                            enemy_type, e, path, alt_path
                        )));
                    }
                }
            }
        };
        
        let chunk = self.lua.load(&code);
        let ai: Table = chunk.eval()?;
        
        if let Ok(update_func) = ai.get::<_, Function>("update") {
            let update_func_static: Function<'static> = unsafe { std::mem::transmute(update_func) };
            self.ai_tables.insert(enemy_type.to_string(), update_func_static);
        }
        
        Ok(())
    }

    pub fn load_all_ai_scripts(&mut self, scripts_dir: &str) -> Result<()> {
        let enemies_dir = format!("{}/enemies", scripts_dir);
        let presets_dir = format!("{}/presets", scripts_dir);
        
        if let Ok(entries) = std::fs::read_dir(&presets_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let preset_name = file_stem.to_string();
                        let script_path = path.to_string_lossy().to_string();
                        
                        match self.load_enemy_ai(&preset_name, &script_path) {
                            Ok(_) => println!("Loaded preset AI: {}", preset_name),
                            Err(e) => eprintln!("Failed to load preset AI {}: {}", preset_name, e),
                        }
                    }
                }
            }
        }
        
        if let Ok(entries) = std::fs::read_dir(&enemies_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let enemy_name = file_stem.to_string();
                        let script_path = path.to_string_lossy().to_string();
                        
                        match self.load_enemy_ai(&enemy_name, &script_path) {
                            Ok(_) => println!("Loaded enemy AI: {}", enemy_name),
                            Err(e) => eprintln!("Failed to load enemy AI {}: {}", enemy_name, e),
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn update_enemy(
        &mut self,
        enemy_type: &str,
        enemy_id: crate::ecs::EntityID,
        enemy: &mut Entity,
        player: &Entity,
        stats: Option<EnemyStats>,
    ) -> Result<(f32, f32)> {
        let now = Instant::now();
        let last_update = self.last_update.entry(enemy_id.0).or_insert(now);
        let delta = now.duration_since(*last_update).as_secs_f32();
        *last_update = now;

        if let Some(update_func) = self.ai_tables.get(enemy_type) {
            let enemy_table = self.lua.create_table()?;
            enemy_table.set("id", enemy.id.0)?;
            enemy_table.set("x", enemy.position_x)?;
            enemy_table.set("y", enemy.position_y)?;
            enemy_table.set("vx", enemy.velocity_x)?;
            enemy_table.set("vy", enemy.velocity_y)?;
            
            if let Some(stats) = stats {
                let stats_table = self.lua.create_table()?;
                stats_table.set("health", stats.health)?;
                stats_table.set("max_health", stats.max_health)?;
                stats_table.set("speed", stats.speed)?;
                stats_table.set("damage", stats.damage)?;
                stats_table.set("experience", stats.experience)?;
                stats_table.set("aggro_range", stats.aggro_range)?;
                stats_table.set("attack_range", stats.attack_range)?;
                stats_table.set("attack_cooldown", stats.attack_cooldown)?;
                stats_table.set("knockback_resistance", stats.knockback_resistance)?;
                stats_table.set("ai_type", stats.ai_type)?;
                stats_table.set("pushable", stats.pushable)?;
                stats_table.set("scale", stats.scale)?;
                enemy_table.set("stats", stats_table)?;
            }

            let player_table = self.lua.create_table()?;
            player_table.set("x", player.position_x)?;
            player_table.set("y", player.position_y)?;
            player_table.set("vx", player.velocity_x)?;
            player_table.set("vy", player.velocity_y)?;
            
            if let Some(collider) = &player.collider {
                let collider_table = self.lua.create_table()?;
                collider_table.set("width", collider.width)?;
                collider_table.set("height", collider.height)?;
                collider_table.set("offset_x", collider.offset_x)?;
                collider_table.set("offset_y", collider.offset_y)?;
                player_table.set("collider", collider_table)?;
            }

            match update_func.call::<_, ()>((enemy_table.clone(), player_table, delta)) {
                Ok(_) => {
                    let vx: f32 = enemy_table.get("vx").unwrap_or(0.0);
                    let vy: f32 = enemy_table.get("vy").unwrap_or(0.0);
                    
                    return Ok((vx, vy));
                }
                Err(e) => {
                    eprintln!("Lua AI execution error for {} (id: {}): {}", enemy_type, enemy_id.0, e);
                    return Ok((0.0, 0.0));
                }
            }
        } else {
            eprintln!("No AI function loaded for enemy type: {}", enemy_type);
            let script_path = format!("assets/scripts/enemies/{}.lua", enemy_type.to_lowercase());
            
            if let Ok(()) = self.load_enemy_ai(enemy_type, &script_path) {
                return self.update_enemy(enemy_type, enemy_id, enemy, player, stats);
            }
        }

        Ok((0.0, 0.0))
    }

    pub fn cleanup_entity(&mut self, entity_id: crate::ecs::EntityID) {
        self.last_update.remove(&entity_id.0);
    }
}