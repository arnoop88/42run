mod shader;
mod mesh;
mod character;
mod level;
mod math;
mod collision;
mod menu;

use glfw::{Action, Context, Key, WindowEvent, MouseButton};
use mesh::Mesh;
use nalgebra::{Vector3};
use menu::{Menu, MenuAction};

enum GameState {
    Menu,
    Game,
}

struct WorldState {
    speed: f32,
    z: f32,
    last_frame_time: f64,
    game_state: GameState,
    screen_width: f32,
    screen_height: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_clicked: bool,
	menu: Menu,
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let (mut window, events) = glfw.create_window(800, 600, "42run", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
	window.set_cursor_pos_polling(true);
	window.set_mouse_button_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let game_shader = shader::Shader::new(
        "shaders/vertex/game.glsl",
        "shaders/fragment/game.glsl"
    ).expect("Failed to load shaders");
	let ui_shader = shader::Shader::new(
		"shaders/vertex/ui.glsl",
		"shaders/fragment/ui.glsl"
	).expect("Failed to load UI shaders");

    let character_mesh = Mesh::cube(Mesh::PLAYER_COLOR);
    let mut character = character::Character::new();
    let mut level_generator = level::LevelGenerator::new();
    let mut world = WorldState {
		speed: 20.0,
		z: 0.0,
		last_frame_time: glfw.get_time(),
		game_state: GameState::Menu,
		screen_width: 800.0,
		screen_height: 600.0,
		mouse_x: 0.0,
		mouse_y: 0.0,
		mouse_clicked: false,
		menu: Menu::new(800.0, 600.0),
    };

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
			match event {
				WindowEvent::FramebufferSize(width, height) => {
					unsafe { gl::Viewport(0, 0, width, height) };
					world.screen_width = width as f32;
					world.screen_height = height as f32;
					world.menu = Menu::new(world.screen_width, world.screen_height);
				}
				WindowEvent::CursorPos(x, y) => {
					world.mouse_x = x as f32;
					world.mouse_y = world.screen_height - y as f32;
				},
				WindowEvent::MouseButton(button, action, _) => {
					if button == MouseButton::Left && action == Action::Press {
						world.mouse_clicked = true;
					}
				},
				_ => handle_window_event(&mut window, event, &mut character),
			}
        }

		match world.game_state {
			GameState::Menu => {
				unsafe {
					gl::ClearColor(0.1, 0.1, 0.1, 1.0);//gl::ClearColor(0.2, 0.4, 0.8, 1.0);
					gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
					ui_shader.use_program();
					world.menu.render(&ui_shader);
				}
				
				if world.mouse_clicked {
					match world.menu.handle_click(world.mouse_x, world.mouse_y) {
						MenuAction::Play => world.game_state = GameState::Game,
						MenuAction::Quit => window.set_should_close(true),
						MenuAction::None => {}
					}
					world.mouse_clicked = false;
				}
			},
			GameState::Game => {
				let current_time = glfw.get_time();
				let delta_time = (current_time - world.last_frame_time) as f32;
				world.last_frame_time = current_time;
				character.update(delta_time);
				world.z += world.speed * delta_time;
				level_generator.update(world.z);
				world.speed = (world.speed + 0.2 * delta_time).min(50.0);
				let mut collision_detected = false;
				let player_aabb = character.get_aabb(world.z);
				//let distance_text = format!("Distance: {:.0}m", world.z);
				//print!("speed: {}, distance: {}\n", world.speed, distance_text);

				unsafe {
					gl::ClearColor(0.1, 0.1, 0.1, 1.0);
					gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

					game_shader.use_program();

					// Camera
					let eye = Vector3::new(0.0, 3.0, -10.0);
					let target = Vector3::new(0.0, 1.5, 0.0);
					let view = math::look_at(eye, target, Vector3::new(0.0, 1.0, 0.0));
					
					let projection = math::perspective(
						45.0f32.to_radians(),
						800.0 / 600.0,
						0.1,
						1000.0
					);

					game_shader.set_mat4("view", &view);
					game_shader.set_mat4("projection", &projection);
					
					for segment in level_generator.segments() {
						let segment_z = segment.position - world.z;
						if segment_z < -25.0 {
							continue;
						}
						
						// Platform
						let model = math::translation(0.0, 0.0, segment_z);
						game_shader.set_mat4("model", &model);
						segment.platform.draw();
					
						// Obstacles
						for obstacle in &segment.obstacles {
							let obstacle_aabb = obstacle.get_aabb();
							if player_aabb.collides(&obstacle_aabb) {
								collision_detected = true;
								break;
							}

							let obstacle_z = obstacle.position.z - world.z;
							if obstacle_z < -25.0 {
								continue;
							}
							
							let model = math::translation(
								obstacle.position.x,
								obstacle.position.y,
								obstacle_z
							);
							game_shader.set_mat4("model", &model);
							obstacle.mesh.draw();
						}
						if collision_detected {
							break;
						}
					}

					if collision_detected {
						world.z = 0.0;
						world.speed = 20.0;
						character = character::Character::new();
						level_generator = level::LevelGenerator::new();
						world.game_state = GameState::Menu;
					}

					let model = math::translation(
						character.position.x,
						character.position.y + 0.5,
						0.0
					);
					game_shader.set_mat4("model", &model);
					character_mesh.draw();
				}
			}
		}
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: WindowEvent, character: &mut character::Character) {
	match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
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