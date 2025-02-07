use nalgebra::{Vector3, Point3};
use rand::Rng;
use crate::mesh::{Mesh, Vertex};
use crate::character::Character;

pub struct LevelSegment {
    pub position: f32,  // Z-axis position
    pub platform: Mesh,
    pub obstacles: Vec<Obstacle>,
}

pub struct Obstacle {
    pub mesh: Mesh,
    pub position: Point3<f32>,
}

pub struct LevelGenerator {
    segments: Vec<LevelSegment>,
    next_z: f32,
    segment_length: f32,
	obstacle_template: Mesh,
}

impl LevelGenerator {
    const SEGMENT_SPACING: f32 = 20.0;
    const VISIBLE_SEGMENTS: usize = 5;

    pub fn new() -> Self {
		let obstacle_template = Mesh::cube(Mesh::OBSTACLE_COLOR);
        let mut generator = Self {
            segments: Vec::new(),
            next_z: -Self::SEGMENT_SPACING,
            segment_length: 20.0,
			obstacle_template,
        };
        
        // Generate initial segments
        for _ in 0..Self::VISIBLE_SEGMENTS {
            generator.generate_segment();
        }
        
        generator
    }

    pub fn generate_segment(&mut self) {
        let segment = LevelSegment {
            position: self.next_z,
            platform: Mesh::platform(/*self.segment_length*/),
            obstacles: self.generate_obstacles(self.next_z),
        };
        
        self.segments.push(segment);
        self.next_z -= Self::SEGMENT_SPACING;
    }

    fn generate_obstacles(&self, z_pos: f32) -> Vec<Obstacle> {
        let mut obstacles = Vec::new();
		let mut rng = rand::thread_rng();
        
        for _ in 0..rng.gen_range(0..3) {
			let lane = rng.gen_range(-1..=1); // -1, 0, or 1
			let x_pos = lane as f32 * Character::LANE_WIDTH;
			
			obstacles.push(Obstacle {
				mesh: self.create_obstacle_mesh(),
				position: Point3::new(
					x_pos,
					0.5,  // Fixed Y position
					z_pos - rng.gen_range(2.0..18.0) // Random Z within segment
				),
			});
		}
		obstacles
    }

    pub fn update(&mut self, character_z: f32) {
        // Remove segments behind character
        while self.segments.first().map(|s| s.position) < Some(character_z + Self::SEGMENT_SPACING) {
            self.segments.remove(0);
            self.generate_segment();
        }
    }

	fn create_obstacle_mesh(&self) -> Mesh {
        // Create a new cube mesh with same parameters as template
        Mesh::cube(Mesh::OBSTACLE_COLOR)
    }

    pub fn segments(&self) -> &[LevelSegment] {
        &self.segments
    }
}