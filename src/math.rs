use nalgebra::{Matrix4, Vector3, Perspective3};

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn to_matrix(&self) -> Matrix4<f32> {
        // Manually compute model matrix using position/rotation/scale
        // (Not using nalgebra's transformation functions!)
    }
}