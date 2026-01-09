use crate::ecs::ECSManager;
use crate::player::Player;
use crate::renderer::Renderer;
use crate::enemy_manager::EnemyManager;
use std::time::{Duration, Instant};

pub struct Game {
    pub player: Player,
    pub ecs_manager: ECSManager,
    pub enemy_manager: EnemyManager,
}

impl Game {
    const fps: u64 = 60;

    pub fn new(renderer: &mut Renderer, width: i32, height: i32) -> Self {
        let mut ecs_manager = ECSManager::new(width, height);
        let mut enemy_manager = EnemyManager::new("assets/settings/enemies.json");

        let mut player = Player::new(renderer);
        ecs_manager.add_entity(player.entity.clone());

        enemy_manager.spawn_enemy("Bug", &mut ecs_manager, renderer, 400, 300);

        Game {
            player,
            ecs_manager,
            enemy_manager,
        }
    }

    pub fn update(&mut self, renderer: &mut Renderer) {
        self.player.update(renderer, &mut self.ecs_manager);
    }

    pub fn run(&mut self, renderer: &mut Renderer) {
        let frame_duration = Duration::from_millis(1000 / Self::fps);

        while renderer.is_open() {
            let start = Instant::now();

            self.update(renderer);
            renderer.render_frame();

            let elapsed = start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }
    }
}

