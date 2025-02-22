use std::fs::File;
use std::io::BufReader;
use gl::types::*;

pub struct Texture {
    pub id: GLuint,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}

impl Texture {
    pub fn new(path: &str) -> Self {
        // Load PNG file
        let file = File::open(path).unwrap_or_else(|_| panic!("Failed to open texture: {}", path));
        let decoder = png::Decoder::new(BufReader::new(file));
        let mut reader = decoder.read_info().unwrap_or_else(|_| panic!("Invalid PNG: {}", path));
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap_or_else(|_| panic!("Failed to read PNG frame: {}", path));
        
		assert_eq!(
            info.color_type,
            png::ColorType::Rgba,
            "Texture must be RGBA format (found {:?} in {})",
            info.color_type,
            path
        );

        // Flip image vertically (OpenGL expects origin at bottom-left)
        let mut flipped = Vec::with_capacity(buf.len());
        let bytes_per_row = info.width as usize * 4; // 4 bytes per pixel (RGBA)
        for row in (0..info.height as usize).rev() {
            let start = row * bytes_per_row;
            let end = start + bytes_per_row;
            flipped.extend_from_slice(&buf[start..end]); // Will panic here if format is wrong
        }

        // Create OpenGL texture
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                info.width as i32,
                info.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                flipped.as_ptr() as *const _
            );
        }

        Texture { id: texture_id }
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}