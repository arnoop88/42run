use nalgebra::{Matrix4, Point3, Vector3, Perspective3};
use crate::character::Character;

pub struct Camera {
    position: Point3<f32>,
    target: Point3<f32>,
    perspective: Perspective3<f32>,
}

impl Camera {
    const FOLLOW_DISTANCE: f32 = 8.0;
    const FOLLOW_HEIGHT: f32 = 3.0;

    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Point3::new(0.0, Self::FOLLOW_HEIGHT, Self::FOLLOW_DISTANCE),
            target: Point3::origin(),
            perspective: Perspective3::new(aspect_ratio, 45.0f32.to_radians(), 0.1, 100.0),
        }
    }

    pub fn update(&mut self, player_z: f32) {
        // Fixed camera position (centered on X axis)
        self.position = Point3::new(
            0.0,  // Always center X
            Self::FOLLOW_HEIGHT,
            player_z - Self::FOLLOW_DISTANCE
        );

        self.target = Point3::new(
            0.0,
            Self::FOLLOW_HEIGHT * 0.5,
            player_z + 10.0
        );
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &Vector3::y())
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        self.perspective.to_homogeneous()
    }
}