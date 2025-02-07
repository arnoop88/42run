use gl::types::*;
use nalgebra::Vector3;
use std::ffi::CString;

#[derive(Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
	pub color: Vector3<f32>,
}

#[derive(Debug)]
pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    indices_count: i32,
}

impl Mesh {
	pub const PLAYER_COLOR: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0); // Red
    pub const OBSTACLE_COLOR: Vector3<f32> = Vector3::new(1.0, 1.0, 0.0); // Yellow
    const ROAD_COLOR: Vector3<f32> = Vector3::new(0.3, 0.3, 0.3); // Dark gray

    pub fn new(vertices: &[Vertex], indices: &[u32]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            // Generate buffers
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            // Bind VAO
            gl::BindVertexArray(vao);

            // Bind and fill VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Bind and fill EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position
            let pos_location = 0;
            gl::EnableVertexAttribArray(pos_location as GLuint);
            gl::VertexAttribPointer(
                pos_location as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                std::ptr::null(),
            );

			// Color
			gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, 3, gl::FLOAT, gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );

            // Unbind VAO
            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            vbo,
            ebo,
            indices_count: indices.len() as i32,
        }
    }

	pub fn platform() -> Self {
        let vertices = vec![
            Vertex { position: Vector3::new(-3.0, -0.1, -10.0), color: Self::ROAD_COLOR },
            Vertex { position: Vector3::new(3.0, -0.1, -10.0), color: Self::ROAD_COLOR },
            Vertex { position: Vector3::new(3.0, -0.1, 10.0), color: Self::ROAD_COLOR },
            Vertex { position: Vector3::new(-3.0, -0.1, 10.0), color: Self::ROAD_COLOR },
        ];

        let indices = vec![
            0, 1, 2,
            2, 3, 0,
        ];

        Mesh::new(&vertices, &indices)
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }

	pub fn cube(color: Vector3<f32>) -> Self {
		let vertices = vec![
			// Front face
			Vertex { position: Vector3::new(-0.5, -0.5, 0.5), color },
            Vertex { position: Vector3::new( 0.5, -0.5, 0.5), color },
            Vertex { position: Vector3::new( 0.5,  0.5, 0.5), color },
            Vertex { position: Vector3::new(-0.5,  0.5, 0.5), color },
			// Back face
			Vertex { position: Vector3::new(-0.5, -0.5, -0.5), color },
			Vertex { position: Vector3::new( 0.5, -0.5, -0.5), color },
			Vertex { position: Vector3::new( 0.5,  0.5, -0.5), color },
			Vertex { position: Vector3::new(-0.5,  0.5, -0.5), color },
		];

		let indices = vec![
			// Front face
			0, 1, 2, 2, 3, 0,
			// Back face
			4, 5, 6, 6, 7, 4,
			// Left face
			4, 0, 3, 3, 7, 4,
			// Right face
			1, 5, 6, 6, 2, 1,
			// Top face
			3, 2, 6, 6, 7, 3,
			// Bottom face
			4, 5, 1, 1, 0, 4,
		];

		Mesh::new(&vertices, &indices)
	}
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}