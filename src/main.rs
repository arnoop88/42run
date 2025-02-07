mod shader;
mod mesh;
mod camera;
mod character;
mod level;

use glfw::{Action, Context, Key, WindowEvent};
use mesh::{Mesh, Vertex};
use nalgebra::{Matrix4, Point3, Vector3};

struct WorldState {
    speed: f32,
    position: f32,
}

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

	let character_mesh = Mesh::cube(Mesh::PLAYER_COLOR);
	let mut character = character::Character::new();
	let mut camera = camera::Camera::new(800.0 / 600.0);
	let mut last_time = glfw.get_time() as f32;
	let mut level_generator = level::LevelGenerator::new();
	let mut world = WorldState {
		speed: 5.0,  // Base movement speed
		position: 0.0,
	};

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut character);
        }

		let current_time = glfw.get_time() as f32;
		let delta_time = current_time - last_time;
		last_time = current_time;
		world.position += world.speed * delta_time;

        unsafe {
			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Activate shader
            shader.use_program();

            // Set up transformation matrices
            let model = Matrix4::new_translation(&Vector3::new(
				character.position.x, 
				character.position.y, 
				0.0 // Keep character at Z=0 (camera follows world)
			));
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

			let model = model * Matrix4::from_euler_angles(0.0, std::f32::consts::PI, 0.0);

            // Set uniforms
            shader.set_mat4("model", &model);
			shader.set_mat4("view", &camera.view_matrix());
			shader.set_mat4("projection", &camera.projection_matrix());

			for segment in level_generator.segments() {
				let segment_z = segment.position + world.position;
				if segment_z > 5.0 { continue; }  // Cull behind camera
				
				// Platform
				let model = Matrix4::new_translation(&Vector3::new(0.0, 0.0, segment_z));
				shader.set_mat4("model", &model);
				segment.platform.draw();
			
				// Obstacles
				for obstacle in &segment.obstacles {
					let model = Matrix4::new_translation(&Vector3::new(
						obstacle.position.x,
						obstacle.position.y,
						obstacle.position.z + world.position
					));
					shader.set_mat4("model", &model);
					obstacle.mesh.draw();
				}
			}
            
			character.update(delta_time);
			camera.update(&character.position);
			character_mesh.draw();
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