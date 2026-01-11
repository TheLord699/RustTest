use crate::sprite::Sprite;

#[derive(Clone)]
pub struct Collider {
    pub width: u32,
    pub height: u32,
    pub offset_x: i32,
    pub offset_y: i32,
}

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub z_order: i32,
    pub sprite: Option<Sprite>,
    pub collider: Option<Collider>,
    pub simple_collisions: bool,
}

impl Entity {
    pub fn new(name: &str, x: i32, y: i32, z_order: i32) -> Self {
        Self {
            name: name.to_string(),
            position_x: x,
            position_y: y,
            velocity_x: 0.0,
            velocity_y: 0.0,
            z_order,
            sprite: None,
            collider: None,
            simple_collisions: false,
        }
    }
    
    pub fn with_collider(mut self, collider: Option<Collider>, simple_collisions: bool) -> Self {
        self.collider = collider;
        if simple_collisions {
            if let Some(ref c) = self.collider {
                self.set_collider_centered(c.width, c.height);
            }
        }
        self.simple_collisions = simple_collisions;
        self
    }
    
    pub fn set_sprite(&mut self, sprite: Sprite) {
        self.sprite = Some(sprite);
    }
    
    pub fn set_collider(&mut self, width: u32, height: u32, offset_x: i32, offset_y: i32) {
        self.collider = Some(Collider { width, height, offset_x, offset_y });
    }
    
    pub fn set_collider_centered(&mut self, width: u32, height: u32) {
        self.set_collider(width, height, -(width as i32) / 2, -(height as i32) / 2);
    }
    
    pub fn move_entity(&mut self, dx: f32, dy: f32) {
        self.position_x = (self.position_x as f32 + dx) as i32;
        self.position_y = (self.position_y as f32 + dy) as i32;
    }
    
    #[inline]
    pub fn get_collider_bounds(&self) -> Option<(i32, i32, i32, i32)> {
        self.collider.as_ref().map(|c| {
            let left = self.position_x + c.offset_x;
            let top = self.position_y + c.offset_y;
            (left, top, left + c.width as i32, top + c.height as i32)
        })
    }
    
    #[inline]
    pub fn check_collision(&self, other: &Entity) -> bool {
        if let (Some((l1, t1, r1, b1)), Some((l2, t2, r2, b2))) = 
            (self.get_collider_bounds(), other.get_collider_bounds()) {
            return r1 > l2 && l1 < r2 && b1 > t2 && t1 < b2;
        }
        false
    }
}

#[derive(Clone)]
pub struct ECSManager {
    pub entities: Vec<Entity>,
    pub width: i32,
    pub height: i32,
}

impl ECSManager {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            entities: Vec::with_capacity(256),
            width,
            height,
        }
    }
    
    pub fn add_entity(&mut self, entity: Entity) -> usize {
        self.entities.push(entity);
        self.entities.len() - 1
    }
    
    pub fn get_entity(&self, idx: usize) -> Option<&Entity> {
        self.entities.get(idx)
    }
    
    pub fn get_entity_mut(&mut self, idx: usize) -> Option<&mut Entity> {
        self.entities.get_mut(idx)
    }
    
    #[inline]
    pub fn get_entity_by_name(&self, name: &str) -> Option<usize> {
        self.entities.iter().position(|e| e.name == name)
    }
    
    pub fn move_entity(&mut self, idx: usize, dx: f32, dy: f32) -> bool {
        if idx >= self.entities.len() { return false; }
        
        let prev_x = self.entities[idx].position_x;
        let prev_y = self.entities[idx].position_y;
        self.entities[idx].move_entity(dx, dy);
        
        for (other_idx, other) in self.entities.iter().enumerate() {
            if other_idx != idx && self.entities[idx].check_collision(other) {
                self.entities[idx].position_x = prev_x;
                self.entities[idx].position_y = prev_y;
                return false;
            }
        }
        
        true
    }
    
    pub fn get_collisions(&self, idx: usize) -> Vec<usize> {
        let mut collisions = Vec::new();
        if let Some(entity) = self.entities.get(idx) {
            for (other_idx, other) in self.entities.iter().enumerate() {
                if other_idx != idx && entity.check_collision(other) {
                    collisions.push(other_idx);
                }
            }
        }
        collisions
    }
}