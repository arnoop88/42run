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
    const VISIBLE_SEGMENTS: usize = 8;

    pub fn new() -> Self {
		let obstacle_template = Mesh::cube(Mesh::OBSTACLE_COLOR);
        let mut generator = Self {
            segments: Vec::new(),
            next_z: 0.0,
            segment_length: 20.0,
			obstacle_template,
        };
        
        // Generate initial segments
        for _ in 0..Self::VISIBLE_SEGMENTS {
			generator.generate_segment();
			generator.next_z += Self::SEGMENT_SPACING;
        }
        
        generator
    }

    pub fn generate_segment(&mut self) {
        let segment = LevelSegment {
            position: self.next_z,
            platform: Mesh::platform(),
            obstacles: self.generate_obstacles(self.next_z),
        };
        
        self.segments.push(segment);
    }

    fn generate_obstacles(&self, z_pos: f32) -> Vec<Obstacle> {
        let mut obstacles = Vec::new();
		let mut rng = rand::thread_rng();
        
        for _ in 0..rng.gen_range(0..3) {
			let lane = rng.gen_range(-1..=1); // -1, 0, or 1
			obstacles.push(Obstacle {
				mesh: self.create_obstacle_mesh(),
				position: Point3::new(
					lane as f32 * Character::LANE_WIDTH,
					0.0,
					z_pos + 30.0
				),
			});
		}
		obstacles
    }

    pub fn update(&mut self, player_z: f32) {
		// Remove segments behind with buffer
		let remove_threshold = player_z - (Self::SEGMENT_SPACING * 1.5);
		while self.segments.first().map(|s| s.position) < Some(remove_threshold) {
			self.segments.remove(0);
		}
	
		// Generate segments with speed anticipation
		let generation_threshold = player_z + (Self::SEGMENT_SPACING * Self::VISIBLE_SEGMENTS as f32);
		while self.next_z < generation_threshold {
			self.generate_segment();
			self.next_z += Self::SEGMENT_SPACING;
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