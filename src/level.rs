use nalgebra::{Point3, Vector3};
use rand::Rng;
use crate::mesh::{Mesh};
use crate::character::Character;
use crate::collision::AABB;

#[derive(Debug, Clone, PartialEq)]
pub enum ObstacleType {
    Cube,
    WideRectangle,
	TallPillar,
	LowBar,
}

pub struct LevelSegment {
    pub position: f32,
    pub platform: Mesh,
	pub wall: Mesh,
    pub obstacles: Vec<Obstacle>,
}

pub struct Obstacle {
    pub mesh: Mesh,
    pub position: Point3<f32>,
	pub obstacle_type: ObstacleType,
}

pub struct LevelGenerator {
    segments: Vec<LevelSegment>,
    next_z: f32,
    segment_length: f32,
	obstacle_template: Mesh,
}

impl LevelGenerator {
    const SEGMENT_SPACING: f32 = 20.0;
    const VISIBLE_SEGMENTS: usize = 15;
	const OBSTACLE_OFFSET: f32 = 15.0;
	pub const LANE_WIDTH: f32 = 2.0;

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
        let obstacles = if self.next_z == 0.0 {
            Vec::new() // No obstacles for first segment
        } else {
            self.generate_obstacles(self.next_z)
        };

        let segment = LevelSegment {
            position: self.next_z,
            platform: Mesh::platform(),
			wall: Mesh::wall(),
            obstacles,
        };
        
        self.segments.push(segment);
    }

    fn generate_obstacles(&self, z_pos: f32) -> Vec<Obstacle> {
        let mut obstacles = Vec::new();
        let mut rng = rand::thread_rng();

        let obstacle_type = match rng.gen_range(0..=3) {
			0 => ObstacleType::Cube,
			1 => ObstacleType::WideRectangle,
			2 => ObstacleType::TallPillar,
            _ => ObstacleType::LowBar,
		};
		
		match obstacle_type {
			ObstacleType::Cube => {
				// Randomly spawn 1 or 2 cubes.
				let num_cubes = rng.gen_range(1..=2);
				for _ in 0..num_cubes {
					let lane = rng.gen_range(-1..=1);
					obstacles.push(Obstacle {
						mesh: Mesh::cube(Mesh::OBSTACLE_COLOR),
						position: Point3::new(lane as f32 * Self::LANE_WIDTH, 0.001, z_pos + Self::OBSTACLE_OFFSET),
						obstacle_type: ObstacleType::Cube,
					});
				}
			}
			ObstacleType::WideRectangle => {
				obstacles.push(Obstacle {
					mesh: Mesh::wide_rectangle(),
					position: Point3::new(0.0, 0.001, z_pos + Self::OBSTACLE_OFFSET),
					obstacle_type: ObstacleType::WideRectangle,
				});
			}
			ObstacleType::TallPillar => {
				let is_left = rng.gen_bool(0.5);
				let x_position = if is_left { -1.0 } else { 1.0 };
				obstacles.push(Obstacle {
					mesh: Mesh::tall_pillar(),
					position: Point3::new(x_position, 0.001, z_pos + Self::OBSTACLE_OFFSET),
					obstacle_type: ObstacleType::TallPillar,
				});
			}
			ObstacleType::LowBar => {
                obstacles.push(Obstacle {
                    mesh: Mesh::low_bar(),
                    position: Point3::new(0.0, 0.8, z_pos + Self::OBSTACLE_OFFSET),
                    obstacle_type: ObstacleType::LowBar,
                });
            }
		}
		obstacles
    }

    pub fn update(&mut self, world_z: f32) {
        let generation_threshold = world_z + 1000.0;
        while self.next_z < generation_threshold {
            self.generate_segment();
            self.next_z += Self::SEGMENT_SPACING;
        }
        
        let remove_threshold = world_z - 30.0;
        self.segments.retain(|s| s.position > remove_threshold);
    }

    pub fn segments(&self) -> &[LevelSegment] {
        &self.segments
    }
}

impl Obstacle {
    pub fn get_aabb(&self) -> AABB {
        let size = match self.obstacle_type {
            ObstacleType::Cube => Vector3::new(1.0, 1.0, 1.0),
            ObstacleType::WideRectangle => Vector3::new(6.0, 1.0, 1.0),
			ObstacleType::TallPillar => Vector3::new(4.0, 2.0, 1.0),
			ObstacleType::LowBar => Vector3::new(6.0, 1.2, 1.0),
        };

        AABB {
            min: Point3::new(
                self.position.x - size.x / 2.0,
                self.position.y,
                self.position.z - size.z / 2.0
            ),
            max: Point3::new(
                self.position.x + size.x / 2.0,
                self.position.y + size.y,
                self.position.z + size.z / 2.0
            ),
        }
    }
}