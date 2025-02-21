use nalgebra::{Vector3, Point3};
use crate::LevelGenerator;

pub struct AABB {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

pub struct Character {
    pub position: Point3<f32>,
    velocity: Vector3<f32>,
    is_grounded: bool,
	is_pressing_down: bool,
    lane: i8,
	target_x: f32,
    target_height: f32,
    pub current_height: f32,
    normal_height: f32,
    squat_height: f32,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

impl Character {
    const JUMP_FORCE: f32 = 8.0;
    const BASE_GRAVITY: f32 = -16.0;
	const FAST_FALL_GRAVITY: f32 = -100.0;
	const LANE_CHANGE_SPEED: f32 = 5.0;

    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            velocity: Vector3::zeros(),
            is_grounded: true,
			is_pressing_down: false,
            lane: 0,
			target_x: 0.0,
			target_height: 1.0,
			current_height: 1.0,
			normal_height: 1.0,
			squat_height: 0.5,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Lateral movement interpolation
		let damping = 1.0 - (-Self::LANE_CHANGE_SPEED * delta_time).exp();
        self.position.x = lerp(
            self.position.x,
            self.target_x,
            damping
        );

		// Squatting
		if self.is_grounded {
            self.target_height = if self.is_pressing_down {
                self.squat_height
            } else {
                self.normal_height
            };
        } else {
            self.target_height = self.normal_height;
        }

		let height_damping = 1.0 - (-Self::LANE_CHANGE_SPEED * delta_time).exp();
		self.current_height = lerp(
			self.current_height,
			self.target_height,
			height_damping
		);
		
		// Gravity
		let gravity = if self.is_pressing_down && !self.is_grounded {
            Self::FAST_FALL_GRAVITY
        } else {
            Self::BASE_GRAVITY
        };
        self.velocity.y += gravity * delta_time;
		self.position.y += self.velocity.y * delta_time;

        // Ground collision
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y = 0.0;
            self.is_grounded = true;
        } else {
            self.is_grounded = false;
        }
    }

	pub fn get_aabb(&self, player_z: f32) -> AABB {
        let half_width = 0.5;
		let height = self.current_height;
        AABB {
            min: Point3::new(
                self.position.x - half_width,
                self.position.y,
                player_z - half_width
            ),
            max: Point3::new(
                self.position.x + half_width,
                self.position.y + height,
                player_z + half_width
            ),
        }
    }

	pub fn move_down(&mut self, state: bool) {
		self.is_pressing_down = state;
	}

    pub fn jump(&mut self) {
        if self.is_grounded {
            self.velocity.y = Self::JUMP_FORCE;
            self.is_grounded = false;
			self.target_height = self.normal_height;
        }
    }

    pub fn move_right(&mut self) {
        if self.lane > -1 {
            self.lane -= 1;
            self.target_x = self.lane as f32 * LevelGenerator::LANE_WIDTH;
        }
    }

    pub fn move_left(&mut self) {
        if self.lane < 1 {
            self.lane += 1;
            self.target_x = self.lane as f32 * LevelGenerator::LANE_WIDTH;
        }
    }
}

impl AABB {
    pub fn collides(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x &&
        self.max.x >= other.min.x &&
        self.min.y <= other.max.y &&
        self.max.y >= other.min.y &&
        self.min.z <= other.max.z &&
        self.max.z >= other.min.z
    }
}