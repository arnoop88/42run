use nalgebra::{Vector3, Point3};
use crate::collision::AABB;

pub struct Character {
    pub position: Point3<f32>,
    velocity: Vector3<f32>,
    is_grounded: bool,
    lane: i8,
	target_x: f32,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

impl Character {
    pub const LANE_WIDTH: f32 = 2.0;
    const JUMP_FORCE: f32 = 10.0;
    const GRAVITY: f32 = -35.0;
	const LANE_CHANGE_SPEED: f32 = 5.0;

    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            velocity: Vector3::zeros(),
            is_grounded: true,
            lane: 0,
			target_x: 0.0,
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
		
		// Gravity
        self.velocity.y += Self::GRAVITY * delta_time;
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
        AABB {
            min: Point3::new(
                self.position.x - half_width,
                self.position.y,
                player_z - half_width
            ),
            max: Point3::new(
                self.position.x + half_width,
                self.position.y + 1.0, // Character height
                player_z + half_width
            ),
        }
    }

    pub fn jump(&mut self) {
        if self.is_grounded {
            self.velocity.y = Self::JUMP_FORCE;
            self.is_grounded = false;
        }
    }

    pub fn move_right(&mut self) {
        if self.lane > -1 {
            self.lane -= 1;
            self.target_x = self.lane as f32 * Self::LANE_WIDTH;
        }
    }

    pub fn move_left(&mut self) {
        if self.lane < 1 {
            self.lane += 1;
            self.target_x = self.lane as f32 * Self::LANE_WIDTH;
        }
    }
}