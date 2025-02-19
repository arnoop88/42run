use std::fs;
use std::ffi::CString;
use gl::types::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Failed to load shader: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Shader compilation failed: {0}")]
    CompileError(String),
    #[error("Program linking failed: {0}")]
    LinkError(String),
}

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self, ShaderError> {
        let vertex_code = fs::read_to_string(vertex_path)?;
        let fragment_code = fs::read_to_string(fragment_path)?;
        
        let vertex_shader = Self::compile_shader(&vertex_code, gl::VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(&fragment_code, gl::FRAGMENT_SHADER)?;
        
        let program = Self::link_program(vertex_shader, fragment_shader)?;
        
        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(Shader { id: program })
    }

    fn compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, ShaderError> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_source = CString::new(source.as_bytes()).unwrap();
        
        unsafe {
            gl::ShaderSource(shader, 1, &c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }

        let mut success = 0;
        unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success) };
        
        if success == 0 {
            let mut len = 0;
            unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buffer = vec![0; len as usize];
            unsafe { gl::GetShaderInfoLog(shader, len, &mut len, buffer.as_mut_ptr() as *mut i8) };
            Err(ShaderError::CompileError(String::from_utf8_lossy(&buffer).into_owned()))
        } else {
            Ok(shader)
        }
    }

    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<GLuint, ShaderError> {
        let program = unsafe { gl::CreateProgram() };
        
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
        }

        let mut success = 0;
        unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut success) };
        
        if success == 0 {
            let mut len = 0;
            unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buffer = vec![0; len as usize];
            unsafe { gl::GetProgramInfoLog(program, len, &mut len, buffer.as_mut_ptr() as *mut i8) };
            Err(ShaderError::LinkError(String::from_utf8_lossy(&buffer).into_owned()))
        } else {
            Ok(program)
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_mat4(&self, name: &str, mat: &nalgebra::Matrix4<f32>) {
        let cname = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, cname.as_ptr());
        gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
    }

	pub unsafe fn set_vec3(&self, name: &str, vec: &nalgebra::Vector3<f32>) {
		let cname = CString::new(name).unwrap();
		let location = gl::GetUniformLocation(self.id, cname.as_ptr());
		gl::Uniform3f(location, vec.x, vec.y, vec.z);
	}

	pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::Uniform1i(location, value);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}