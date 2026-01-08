mod sprite;
mod renderer;
mod ecs;
mod player;
mod game;
mod enemy_manager;

use crate::game::Game;
use crate::renderer::Renderer;

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    let mut renderer = Renderer::new("Game Engine", screen_width as usize, screen_height as usize);
    let mut game = Game::new(&mut renderer, screen_width as i32, screen_height as i32);

    game.run(&mut renderer);
}