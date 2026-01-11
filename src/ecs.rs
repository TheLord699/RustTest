use crate::sprite::Sprite;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityID(pub usize);

#[derive(Clone)]
pub struct Collider {
    pub width: u32,
    pub height: u32,
    pub offset_x: i32,
    pub offset_y: i32,
}

#[derive(Clone)]
pub struct Entity {
    pub id: EntityID,
    pub position_x: i32,
    pub position_y: i32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub z_order: i32,
    pub sprite: Option<Sprite>,
    pub collider: Option<Collider>,
    pub simple_collisions: bool,
    pub pushable: bool,
    pub solid: bool,
    pub mass: f32,
}

impl Entity {
    pub fn new(id: EntityID, _name: &str, x: i32, y: i32, z_order: i32) -> Self {
        Self {
            id,
            position_x: x,
            position_y: y,
            velocity_x: 0.0,
            velocity_y: 0.0,
            z_order,
            sprite: None,
            collider: None,
            simple_collisions: false,
            pushable: true,
            solid: true,
            mass: 1.0,
        }
    }
    
    pub fn with_solid(mut self, solid: bool) -> Self {
        self.solid = solid;
        self
    }
    
    pub fn with_pushable(mut self, pushable: bool) -> Self {
        self.pushable = pushable;
        self
    }
    
    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
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
    entities: HashMap<EntityID, Entity>,
    next_id: usize,
}

impl ECSManager {
    pub fn new(_width: i32, _height: i32) -> Self {
        Self {
            entities: HashMap::with_capacity(256),
            next_id: 1,
        }
    }
    
    pub fn add_entity(&mut self, entity: Entity) -> EntityID {
        let id = entity.id;
        self.entities.insert(id, entity);
        id
    }
    
    pub fn create_entity(&mut self, name: &str, x: i32, y: i32, z_order: i32) -> EntityID {
        let id = EntityID(self.next_id);
        self.next_id += 1;
        let entity = Entity::new(id, name, x, y, z_order);
        self.entities.insert(id, entity);
        id
    }
    
    pub fn get_entity(&self, id: EntityID) -> Option<&Entity> {
        self.entities.get(&id)
    }
    
    pub fn get_entity_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }
    
    pub fn iter_entity_ids(&self) -> impl Iterator<Item = EntityID> + '_ {
        self.entities.keys().copied()
    }
    
    pub fn move_entity(&mut self, id: EntityID, dx: f32, dy: f32) -> bool {
        // First, check if the entity exists
        let entity = match self.entities.get(&id) {
            Some(e) => e,
            None => return false,
        };
        
        // Calculate new position
        let new_x = (entity.position_x as f32 + dx) as i32;
        let new_y = (entity.position_y as f32 + dy) as i32;
        
        // Get the entity's collider bounds at new position
        let entity_bounds = match entity.get_collider_bounds() {
            Some(bounds) => bounds,
            None => {
                // Entity has no collider, just move it
                if let Some(entity_mut) = self.entities.get_mut(&id) {
                    entity_mut.position_x = new_x;
                    entity_mut.position_y = new_y;
                }
                return true;
            }
        };
        
        // Adjust bounds for new position
        let dx_i = new_x - entity.position_x;
        let dy_i = new_y - entity.position_y;
        let (l1, t1, r1, b1) = entity_bounds;
        let new_l1 = l1 + dx_i;
        let new_t1 = t1 + dy_i;
        let new_r1 = r1 + dx_i;
        let new_b1 = b1 + dy_i;
        
        // Check for collisions with all other entities
        let mut can_move = true;
        let mut entities_to_push = Vec::new();
        
        for (&other_id, other) in &self.entities {
            if other_id == id {
                continue;
            }
            
            // Skip non-solid entities
            if !other.solid {
                continue;
            }
            
            // Get other entity's bounds
            if let Some((l2, t2, r2, b2)) = other.get_collider_bounds() {
                // Check for collision at new position
                if new_r1 > l2 && new_l1 < r2 && new_b1 > t2 && new_t1 < b2 {
                    // Collision detected!
                    if other.pushable {
                        // Calculate overlap to determine push direction
                        let overlap_left = new_r1 - l2;
                        let overlap_right = r2 - new_l1;
                        let overlap_top = new_b1 - t2;
                        let overlap_bottom = b2 - new_t1;
                        
                        // Find the minimum overlap direction
                        let overlaps = [
                            (overlap_left, "right"),
                            (overlap_right, "left"),
                            (overlap_top, "down"),
                            (overlap_bottom, "up"),
                        ];
                        
                        let min_overlap = overlaps.iter()
                            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                            .unwrap();
                        
                        // Calculate push amount based on mass ratio
                        let push_ratio = entity.mass / (entity.mass + other.mass);
                        let push_amount = min_overlap.0 as f32 * push_ratio;
                        
                        // Store push information
                        match min_overlap.1 {
                            "right" => entities_to_push.push((other_id, push_amount, 0.0)),
                            "left" => entities_to_push.push((other_id, -push_amount, 0.0)),
                            "down" => entities_to_push.push((other_id, 0.0, push_amount)),
                            "up" => entities_to_push.push((other_id, 0.0, -push_amount)),
                            _ => {}
                        }
                    } else {
                        // Collision with unpushable entity - can't move
                        can_move = false;
                        break;
                    }
                }
            }
        }
        
        // If we can move, update position
        if can_move {
            if let Some(entity_mut) = self.entities.get_mut(&id) {
                entity_mut.position_x = new_x;
                entity_mut.position_y = new_y;
            }
            
            // Process pushed entities (with a push depth limit to prevent chains)
            if !entities_to_push.is_empty() {
                self.process_pushed_entities(entities_to_push, id);
            }
            
            true
        } else {
            false
        }
    }
    
    // Helper method to process pushed entities without causing chain reactions
    fn process_pushed_entities(&mut self, entities_to_push: Vec<(EntityID, f32, f32)>, _original_pusher: EntityID) {
        // Only process a limited number of pushes to prevent chains
        const MAX_PUSH_DEPTH: usize = 1;
        
        for (i, (other_id, push_x, push_y)) in entities_to_push.iter().enumerate() {
            if i >= MAX_PUSH_DEPTH {
                break;
            }
            
            // Only push a small amount
            let push_x = push_x.min(10.0).max(-10.0);
            let push_y = push_y.min(10.0).max(-10.0);
            
            // Simple move without pushing others
            self.simple_move_entity(*other_id, push_x, push_y);
        }
    }
    
    // Helper method for simple movement without pushing other entities
    fn simple_move_entity(&mut self, id: EntityID, dx: f32, dy: f32) -> bool {
        let entity = match self.entities.get(&id) {
            Some(e) => e,
            None => return false,
        };
        
        let new_x = (entity.position_x as f32 + dx) as i32;
        let new_y = (entity.position_y as f32 + dy) as i32;
        
        // Check for collisions at new position - only with unpushable entities
        if let Some(entity_bounds) = entity.get_collider_bounds() {
            let dx_i = new_x - entity.position_x;
            let dy_i = new_y - entity.position_y;
            let (l1, t1, r1, b1) = entity_bounds;
            let new_l1 = l1 + dx_i;
            let new_t1 = t1 + dy_i;
            let new_r1 = r1 + dx_i;
            let new_b1 = b1 + dy_i;
            
            for (&other_id, other) in &self.entities {
                if other_id == id {
                    continue;
                }
                
                // Only check collisions with solid, unpushable entities
                if other.solid && !other.pushable {
                    if let Some((l2, t2, r2, b2)) = other.get_collider_bounds() {
                        if new_r1 > l2 && new_l1 < r2 && new_b1 > t2 && new_t1 < b2 {
                            // Can't move into unpushable entity
                            return false;
                        }
                    }
                }
            }
        }
        
        // Update position
        if let Some(entity_mut) = self.entities.get_mut(&id) {
            entity_mut.position_x = new_x;
            entity_mut.position_y = new_y;
        }
        
        true
    }
    
    pub fn iter_entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }
    
    pub fn iter_entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }
    
    pub fn remove_entity(&mut self, id: EntityID) -> Option<Entity> {
        self.entities.remove(&id)
    }
    
    pub fn clear(&mut self) {
        self.entities.clear();
    }
}