use crate::ecs::ECSManager;
use crate::player::Player;
use crate::renderer::Renderer;
use crate::enemy_manager::EnemyManager;

pub struct Game {
    pub player: Player,
    pub ecs_manager: ECSManager,
    pub enemy_manager: EnemyManager,
}

impl Game {
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
        while renderer.is_open() {
            self.update(renderer);
            renderer.render_frame();
        }
    }
}
