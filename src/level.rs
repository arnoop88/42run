// src/level.rs
pub struct LevelSegment {
    pub position: f32,
    pub obstacles: Vec<Obstacle>,
}

pub struct LevelGenerator {
    segments: Vec<LevelSegment>,
    next_z: f32,
}

impl LevelGenerator {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            next_z: 0.0,
        }
    }

    pub fn generate(&mut self) {
        let segment = LevelSegment {
            position: self.next_z,
            obstacles: vec![/* Add obstacles here */],
        };
        self.segments.push(segment);
        self.next_z -= 10.0; // Move generation forward
    }
}