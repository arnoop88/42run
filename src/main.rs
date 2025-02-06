mod shader;
mod mesh;
mod camera;
mod character;

use glfw::{Action, Context, Key, WindowEvent};
use mesh::{Mesh, Vertex};
use nalgebra::{Matrix4, Point3, Vector3};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let (mut window, events) = glfw.create_window(800, 600, "42run", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);
	
	unsafe {
		gl::Enable(gl::DEPTH_TEST);
		gl::DepthFunc(gl::LESS);
	}

	let shader = shader::Shader::new(
        "shaders/vertex/basic.glsl",
        "shaders/fragment/basic.glsl"
    ).expect("Failed to load shaders");

	let character_mesh = Mesh::cube();
	let mut character = character::Character::new();
	let mut camera = camera::Camera::new(800.0 / 600.0);
	let mut last_time = glfw.get_time() as f32;

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut character);
        }

		let current_time = glfw.get_time() as f32;
		let delta_time = current_time - last_time;
		last_time = current_time;

        unsafe {
			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            // Activate shader
            shader.use_program();
            
            // Set up transformation matrices
            let model = Matrix4::new_translation(&character.position.coords);
            // let view = nalgebra::Matrix4::look_at_rh(
            //     &nalgebra::Point3::new(0.0, 1.5, 3.0),
            //     &nalgebra::Point3::origin(),
            //     &nalgebra::Vector3::y()
            // );
            // let projection = nalgebra::Perspective3::new(
            //     800.0 / 600.0,
            //     45.0f32.to_radians(),
            //     0.1,
            //     100.0
            // ).to_homogeneous();
            
            // Set uniforms
            shader.set_mat4("model", &model);
			shader.set_mat4("view", &camera.view_matrix());
			shader.set_mat4("projection", &camera.projection_matrix());
            
            character_mesh.draw();
			character.update(delta_time);
			camera.update(&character.position);
        }

        window.swap_buffers();
		glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: WindowEvent, character: &mut character::Character) {
    match event {
        glfw::WindowEvent::Key(Key::Escape,  _, Action::Press, _) |
		glfw::WindowEvent::Key(Key::Q,  _, Action::Press, _) => window.set_should_close(true),
		glfw::WindowEvent::Key(Key::Left,  _, Action::Press, _) |
		glfw::WindowEvent::Key(Key::A,  _, Action::Press, _) => character.move_left(),
		glfw::WindowEvent::Key(Key::Right,  _, Action::Press, _) |
		glfw::WindowEvent::Key(Key::D,  _, Action::Press, _) => character.move_right(),
		glfw::WindowEvent::Key(Key::Space,  _, Action::Press, _) |
		glfw::WindowEvent::Key(Key::Up,  _, Action::Press, _) |
		glfw::WindowEvent::Key(Key::W,  _, Action::Press, _) => character.jump(),
		_ => {}
    }
}