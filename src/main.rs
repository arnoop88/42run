mod shader;
mod mesh;
mod character;
mod level;
mod math;
mod collision;
mod menu;
mod controls;
mod game;
mod texture;
mod pause;
mod game_over;

use glfw::{Action, Context, WindowEvent, MouseButton};
use crate::mesh::Mesh;
use crate::controls::handle_keys;
use crate::level::LevelGenerator;
use crate::game::{new_game, play};
use crate::menu::{Menu, MenuAction};
use crate::pause::{Pause, PauseAction};
use crate::game_over::{GameOver, GameOverAction};

enum GameState {
    Menu,
    Playing,
	GameOver,
	Paused,
}

struct WorldState {
    speed: f32,
    z: f32,
    last_frame_time: f64,
    screen_width: f32,
    screen_height: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_clicked: bool,
	menu: Menu,
	pause: Pause,
	game_over: GameOver,
	level: LevelGenerator,
	pause_start_time: f64,
    total_pause_time: f64,
	high_score: i32,
	record: bool,
}

fn main() {
	const SCREEN_WIDTH: f32 = 800.0;
	const SCREEN_HEIGHT: f32 = 600.0;
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let (mut window, events) = glfw.create_window(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, "42run", glfw::WindowMode::Windowed)
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

    let game_shader = shader::Shader::new("shaders/vertex/game.glsl", "shaders/fragment/game.glsl").expect("Failed to load shaders");
	let ui_shader = shader::Shader::new("shaders/vertex/ui.glsl", "shaders/fragment/ui.glsl").expect("Failed to load UI shaders");
	let text_shader = shader::Shader::new("shaders/vertex/text.glsl", "shaders/fragment/text.glsl").expect("Failed to load text shaders");

    let character_mesh = Mesh::cube(Mesh::PLAYER_COLOR);
	let mut game_state = GameState::Menu;
    let mut character = character::Character::new();
    let mut world = WorldState {
		speed: 20.0,
		z: 0.0,
		last_frame_time: glfw.get_time(),
		screen_width: SCREEN_WIDTH,
		screen_height: SCREEN_HEIGHT,
		mouse_x: 0.0,
		mouse_y: 0.0,
		mouse_clicked: false,
		menu: Menu::new(SCREEN_WIDTH, SCREEN_HEIGHT),
		pause: Pause::new(SCREEN_WIDTH, SCREEN_HEIGHT),
		game_over: GameOver::new(SCREEN_WIDTH, SCREEN_HEIGHT),
		level: level::LevelGenerator::new(),
		pause_start_time: 0.0,
		total_pause_time: 0.0,
		high_score: 0,
		record: false,
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
				_ => handle_keys(&mut window, event, &mut game_state, &mut character, &mut world, &glfw),
			}
        }

		match game_state {
			GameState::Menu => {
				unsafe {
					world.menu.render(&ui_shader, &text_shader);
				}
				if world.mouse_clicked {
					match world.menu.handle_click(world.mouse_x, world.mouse_y) {
						MenuAction::Play => new_game(&mut game_state, &mut character, &mut world, &glfw),
						MenuAction::Quit => window.set_should_close(true),
						MenuAction::None => {}
					}
					world.mouse_clicked = false;
				}
			},
			GameState::Playing => {
				let current_time = glfw.get_time();
				let adjusted_time = current_time - world.total_pause_time;
				let delta_time = (adjusted_time - world.last_frame_time) as f32;
				world.last_frame_time = adjusted_time;
				character.update(delta_time);
				play(&mut world, &mut character, &mut game_state, &game_shader, &character_mesh, &text_shader, delta_time);
			}
			GameState::Paused => {
				unsafe {
					world.pause.render(&ui_shader, &text_shader);
				}
				if world.mouse_clicked {
					match world.pause.handle_click(world.mouse_x, world.mouse_y) {
						PauseAction::Resume => {
							game_state = GameState::Playing;
							world.total_pause_time += glfw.get_time() - world.pause_start_time;
						}
						PauseAction::Quit => game_state = GameState::Menu,
						PauseAction::None => {}
					}
					world.mouse_clicked = false;
				}
			}
			GameState::GameOver => {
				if world.z as i32 / 10 > world.high_score {
					world.high_score = world.z as i32 / 10;
					world.record = true;
				}
				unsafe {
					world.game_over.render(&ui_shader, &text_shader, world.high_score, world.record);
				}
				if world.mouse_clicked {
					match world.game_over.handle_click(world.mouse_x, world.mouse_y) {
						GameOverAction::NewGame => new_game(&mut game_state, &mut character, &mut world, &glfw),
						GameOverAction::Quit => game_state = GameState::Menu,
						GameOverAction::None => {}
					}
					world.mouse_clicked = false;
				}
			}
		}
        window.swap_buffers();
        glfw.poll_events();
    }
}