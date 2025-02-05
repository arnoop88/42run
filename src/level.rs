// src/level.rs
pub struct LevelSegment {
    pub geometry: Mesh,
    pub obstacles: Vec<Obstacle>,
}

pub struct LevelGenerator {
    segments: Vec<LevelSegment>,
    current_offset: f32,
}

impl LevelGenerator {
    pub fn generate_next(&mut self) {
        // Combine premade 3D elements in random configurations
    }
}