use gl::types::*;
use nalgebra::{Vector3, Vector2};

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
	pub color: Vector3<f32>,
	pub tex_coords: Vector2<f32>,
}

pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    pub indices_count: i32,
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
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

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

            // Position (location = 0)
            gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as GLsizei,
				std::ptr::null(),
			);

			// Color (location = 1)
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as GLsizei,
				(3 * std::mem::size_of::<f32>()) as *const _,
			);

			// Texture Coordinates (location = 2)
			gl::EnableVertexAttribArray(2);
			gl::VertexAttribPointer(
				2,
				2,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as GLsizei,
				(6 * std::mem::size_of::<f32>()) as *const _,
			);

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
            Vertex { position: Vector3::new(-3.0, 0.0, -20.0), color: Self::ROAD_COLOR, tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new(3.0, 0.0, -20.0), color: Self::ROAD_COLOR, tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new(3.0, 0.0, 20.0), color: Self::ROAD_COLOR, tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new(-3.0, 0.0, 20.0), color: Self::ROAD_COLOR, tex_coords: Vector2::zeros() },
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
			Vertex { position: Vector3::new(-0.5,  0.0,  0.5), color, tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new( 0.5,  0.0,  0.5), color, tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new( 0.5,  0.0, -0.5), color, tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new(-0.5,  0.0, -0.5), color, tex_coords: Vector2::zeros() },
			// Back face
			Vertex { position: Vector3::new(-0.5,  1.0,  0.5), color, tex_coords: Vector2::zeros() },
			Vertex { position: Vector3::new( 0.5,  1.0,  0.5), color, tex_coords: Vector2::zeros() },
			Vertex { position: Vector3::new( 0.5,  1.0, -0.5), color, tex_coords: Vector2::zeros() },
			Vertex { position: Vector3::new(-0.5,  1.0, -0.5), color, tex_coords: Vector2::zeros() },
		];

		let indices = vec![
			0, 1, 2, 2, 3, 0,
			4, 5, 6, 6, 7, 4,
			4, 0, 3, 3, 7, 4,
			1, 5, 6, 6, 2, 1,
			4, 5, 1, 1, 0, 4,
		];

		Mesh::new(&vertices, &indices)
	}

	pub fn quad_2d() -> Self {
        let vertices = vec![
            Vertex { position: Vector3::new(0.0, 0.0, 0.0), color: Vector3::zeros(), tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new(1.0, 0.0, 0.0), color: Vector3::zeros(), tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new(1.0, 1.0, 0.0), color: Vector3::zeros(), tex_coords: Vector2::zeros() },
            Vertex { position: Vector3::new(0.0, 1.0, 0.0), color: Vector3::zeros(), tex_coords: Vector2::zeros() },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Mesh::new(&vertices, &indices)
    }

	pub fn text(text: &str) -> Mesh {
		let mut vertices = Vec::new();
		let mut indices = Vec::new();
		let char_width = 1.0 / 16.0;
		let char_height = 1.0 / 16.0;
		let scale = 1.0;
		let mut x_offset = 0.0;
	
		for (i, c) in text.chars().enumerate() {
			let ascii = c as u32;
			// Shift index so that the atlas cell 0 corresponds to ASCII 32.
			let index = ascii.checked_sub(32).unwrap_or(0);
			let grid_x = (index % 16) as f32;
			let grid_y = (index / 16) as f32;
			// Because texture.rs flips the image vertically, we need to flip the grid Y:
			let grid_y_effective = 15.0 - grid_y;  // For a 16x16 grid (indices 0..15)
	
			let u = grid_x * char_width;
			let v = grid_y_effective * char_height;
			let u_right = u + char_width;
			let v_top = v + char_height;
	
			vertices.extend_from_slice(&[
				Vertex {
					position: Vector3::new(x_offset, 0.0, 0.0),
					color: Vector3::zeros(),
					tex_coords: Vector2::new(u, v),
				},
				Vertex {
					position: Vector3::new(x_offset + scale, 0.0, 0.0),
					color: Vector3::zeros(),
					tex_coords: Vector2::new(u_right, v),
				},
				Vertex {
					position: Vector3::new(x_offset + scale, scale, 0.0),
					color: Vector3::zeros(),
					tex_coords: Vector2::new(u_right, v_top),
				},
				Vertex {
					position: Vector3::new(x_offset, scale, 0.0),
					color: Vector3::zeros(),
					tex_coords: Vector2::new(u, v_top),
				},
			]);
	
			let base = (i * 4) as u32;
			indices.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 3, base]);
			x_offset += scale * 0.8;
		}
	
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