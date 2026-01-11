use crate::ecs::ECSManager;
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
    pub enemy_manager: EnemyManager,
    pub camera: Camera,
    #[allow(unused)]
    stream: OutputStream,
    #[allow(unused)]
    sink: Sink,
}

impl Game {
    const FPS: u64 = 60;
    
    pub fn new(renderer: &mut Renderer, width: i32, height: i32) -> Self {
        let mut ecs_manager = ECSManager::new(width, height);
        let enemy_manager = EnemyManager::new("assets/settings/enemies.json");
        
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        let test_sound = Decoder::new(BufReader::new(File::open("assets/audio/music/aphex.mp3").unwrap())).unwrap();
        sink.append(test_sound);
        
        let player = Player::new(renderer);
        ecs_manager.add_entity(player.entity.clone());
        enemy_manager.spawn_enemy("Bug", &mut ecs_manager, renderer, 400, 300);
        
        let mut camera = Camera::new(width, height);
        camera.follow(player.entity.position_x, player.entity.position_y);
        
        let mut game = Game {
            player,
            ecs_manager,
            enemy_manager,
            camera,
            stream,
            sink,
        };
        
        renderer.set_camera(game.camera.x, game.camera.y);
        game
    }
    
    pub fn update(&mut self, renderer: &mut Renderer) {
        self.player.update(renderer, &mut self.ecs_manager);
        self.camera.follow_smooth(
            self.player.entity.position_x,
            self.player.entity.position_y,
            0.1,
        );
        renderer.set_camera(self.camera.x, self.camera.y);
    }
    
    pub fn run(&mut self, renderer: &mut Renderer) {
        let frame_duration = Duration::from_millis(1000 / Self::FPS);
        
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