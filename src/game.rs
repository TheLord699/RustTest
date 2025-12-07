use crate::ecs::{ECSManager, Entity};
use crate::player::Player;
use crate::renderer::Renderer;
use crate::sprite::Sprite;

pub struct Game {
    pub player: Player,
    pub ecs_manager: ECSManager,
}

impl Game {
    pub fn new(renderer: &mut Renderer, width: i32, height: i32) -> Self {
        let mut ecs_manager = ECSManager::new(width, height);

        let mut player = Player::new(renderer);
        ecs_manager.add_entity(player.entity.clone());

        // testing
        let mut bug = Entity::new("bug", 400, 300, 1);
        let bug_sprite = Sprite::load("assets/sprites/bug.png").scale(3);
        bug.set_sprite(bug_sprite.clone());
        
        let bug_collider_width = (bug_sprite.width * 2 / 3) as u32;
        let bug_collider_height = (bug_sprite.height * 2 / 3) as u32;
        bug.set_collider_centered(bug_collider_width, bug_collider_height);
        ecs_manager.add_entity(bug);
        renderer.add_sprite_instance(
            "bug",
            crate::renderer::SpriteInstance {
                sprite: bug_sprite,
                position_x: 400,
                position_y: 300,
                z_order: 1,
            },
        );
        // end testing

        Game {
            player,
            ecs_manager,
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