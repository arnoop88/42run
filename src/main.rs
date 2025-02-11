mod shader;
mod mesh;
mod camera;
mod character;
mod level;
mod math;

use glfw::{Action, Context, Key, WindowEvent};
use mesh::Mesh;
use nalgebra::{Point3, Vector3};
use math::{translation, perspective, look_at};

struct WorldState {
    speed: f32,
    player_z: f32,
    last_frame_time: f64,
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
    let mut level_generator = level::LevelGenerator::new();
    let mut world = WorldState {
        speed: 5.0,
        player_z: 0.0,
        last_frame_time: glfw.get_time(),
    };

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut character);
        }

        let current_time = glfw.get_time();
        let delta_time = (current_time - world.last_frame_time) as f32;
        world.last_frame_time = current_time;
		character.update(delta_time);
        world.player_z += world.speed * delta_time;
		level_generator.update(world.player_z);
        world.speed = (world.speed + 0.05 * delta_time).min(10.0);
		let distance_text = format!("Distance: {:.0}m", world.player_z);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();

			// Camera
			let eye = Vector3::new(0.0, 3.0, world.player_z - 10.0);
			let target = Vector3::new(0.0, 1.0, world.player_z + 10.0);
			let view = math::look_at(eye, target, Vector3::new(0.0, 1.0, 0.0));
            
            let projection = perspective(
                45.0f32.to_radians(),
                800.0 / 600.0,
                0.1,
                1000.0
            );

            shader.set_mat4("view", &view);
            shader.set_mat4("projection", &projection);
            
            for segment in level_generator.segments() {
                let segment_z = segment.position - world.player_z;
                if segment_z < -25.0 {
                    continue;
                }
                
                // Platform
                let model = translation(0.0, 0.0, segment_z);
                shader.set_mat4("model", &model);
                segment.platform.draw();
            
                // Obstacles
                for obstacle in &segment.obstacles {
                    let obstacle_z = obstacle.position.z - world.player_z;
					if obstacle_z < -25.0 {
						continue;
					}
                    
                    let model = translation(
                        obstacle.position.x,
                        obstacle.position.y,
                        obstacle_z
                    );
                    shader.set_mat4("model", &model);
                    obstacle.mesh.draw();
                }
            }
            
            // Character using custom translation
            let model = translation(
                character.position.x,
                character.position.y,// + 0.5,
                world.player_z
            );
            shader.set_mat4("model", &model);
            character_mesh.draw();
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: WindowEvent, character: &mut character::Character) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) |
        glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) |
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => character.move_left(),
        glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) |
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => character.move_right(),
        glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) |
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) |
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => character.jump(),
        _ => {}
    }
}