use nalgebra::{Matrix4, Vector3};

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn scaling(sx: f32, sy: f32, sz: f32) -> Matrix4<f32> {
    Matrix4::new(
        sx,  0.0, 0.0, 0.0,
        0.0, sy,  0.0, 0.0,
        0.0, 0.0, sz,  0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let f = 1.0 / (fovy / 2.0).tan();
    let range_inv = 1.0 / (near - far);

    Matrix4::new(
        f / aspect, 0.0, 0.0,                0.0,
        0.0,        f,   0.0,                0.0,
        0.0,        0.0, (near + far) * range_inv,  (2.0 * near * far) * range_inv,
        0.0,        0.0, -1.0,               0.0
    )
}

pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4<f32> {
    let tx = -(right + left) / (right - left);
    let ty = -(top + bottom) / (top - bottom);
    let tz = -(far + near) / (far - near);

    Matrix4::new(
        2.0 / (right - left), 0.0,                 0.0,                 tx,
        0.0,                  2.0 / (top - bottom), 0.0,                 ty,
        0.0,                  0.0,                 -2.0 / (far - near), tz,
        0.0,                  0.0,                 0.0,                 1.0
    )
}

pub fn look_at(eye: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let f = (target - eye).normalize();
    let s = f.cross(&up).normalize();
    let u = s.cross(&f);

    let orientation = Matrix4::new(
        s.x,  s.y,  s.z, 0.0,
        u.x,  u.y,  u.z, 0.0,
        -f.x, -f.y, -f.z, 0.0,
        0.0,  0.0,  0.0,  1.0
    );

    let translation = translation(-eye.x, -eye.y, -eye.z);
    orientation * translation
}