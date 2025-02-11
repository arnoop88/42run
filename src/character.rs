use nalgebra::{Vector3, Point3};

pub struct Character {
    pub position: Point3<f32>,
    velocity: Vector3<f32>,
    is_grounded: bool,
    lane: i8, // -1 = left, 0 = center, 1 = right
}

impl Character {
    pub const LANE_WIDTH: f32 = 2.0;
    const JUMP_FORCE: f32 = 10.0;
    const GRAVITY: f32 = -25.0;
    const MOVE_SPEED: f32 = 8.0;

    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            velocity: Vector3::zeros(),
            is_grounded: true,
            lane: 0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Apply gravity
        self.velocity.y += Self::GRAVITY * delta_time;
        self.position.y += self.velocity.y * delta_time;

        // Ground collision (platform at Y=0)
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y = 0.0;
            self.is_grounded = true;
        } else {
            self.is_grounded = false;
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
            self.position.x = self.lane as f32 * Self::LANE_WIDTH;
        }
    }

    pub fn move_left(&mut self) {
        if self.lane < 1 {
            self.lane += 1;
            self.position.x = self.lane as f32 * Self::LANE_WIDTH;
        }
    }
}