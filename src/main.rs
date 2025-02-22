mod shader;
mod mesh;
mod character;
mod level;
mod math;
mod menu;
mod controls;
mod game;
mod texture;
mod pause;
mod game_over;
mod maps;
mod skins;
mod save_data;

use glfw::{Action, Context, WindowEvent, MouseButton};
use std::collections::HashMap;
use crate::mesh::Mesh;
use crate::controls::handle_keys;
use crate::level::LevelGenerator;
use crate::game::{new_game, play};
use crate::menu::{Menu, MenuAction, render_message};
use crate::pause::{Pause, PauseAction};
use crate::game_over::{GameOver, GameOverAction};
use crate::maps::{MapSelect, MapAction, Maps};
use crate::skins::{SkinSelect, SkinAction, Skins};
use crate::texture::Texture;
use crate::save_data::{save_progress, load_progress, extract_save_data};

#[derive(Clone)]
enum GameState {
    Menu,
	MapSelect,
    SkinSelect,
	ShowMessage(String),
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
	record: bool,
	current_skin: Skins,
	current_map: Maps,
	textures: HashMap<String, Texture>,
	unlocked_maps: HashMap<String, bool>,
    unlocked_skins: HashMap<String, bool>,
    quest_progress: HashMap<String, i32>,
}

impl WorldState {
	fn change_map(&mut self) {
		let map_path = match &self.current_map {
            Maps::Cave(path) | Maps::Temple(path) => format!("assets/textures/maps/{}", path),
            Maps::None => String::from("assets/textures/maps/cave.png"),
        };
		
		self.textures.insert("floor".into(), Texture::new(&format!("{}/floor.png", map_path)));
		self.textures.insert("wall".into(), Texture::new(&format!("{}/wall.png", map_path)));
		self.textures.insert("ceiling".into(), Texture::new(&format!("{}/ceiling.png", map_path)));
		self.textures.insert("cube".into(), Texture::new(&format!("{}/cube.png", map_path)));
		self.textures.insert("lowBar".into(), Texture::new(&format!("{}/lowBar.png", map_path)));
		self.textures.insert("tallWall".into(), Texture::new(&format!("{}/tallWall.png", map_path)));
		self.textures.insert("highBar".into(), Texture::new(&format!("{}/highBar.png", map_path)));
	}

	fn change_skin(&mut self) {
		let skin_path = match &self.current_skin {
            Skins::Red(path) | Skins::Troll(path) | Skins::Dirt(path) | Skins::Stone(path) |
            Skins::Diamond(path) | Skins::Emerald(path) | Skins::Arcane(path) => format!("assets/textures/skins/{}", path),
            Skins::None => String::from("assets/textures/skins/red.png"),
        };

		self.textures.insert("skin".into(), Texture::new(&format!("{}.png", skin_path)));
	}
}

fn main() {
	const SCREEN_WIDTH: f32 = 1024.0;
	const SCREEN_HEIGHT: f32 = 768.0;
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

	let mut textures = HashMap::new();
	textures.insert("font".into(), Texture::new("assets/fonts/MinecraftRegular.png"));
	textures.insert("skin".into(), Texture::new("assets/textures/skins/red.png"));
	textures.insert("floor".into(), Texture::new("assets/textures/maps/cave/floor.png"));
	textures.insert("wall".into(), Texture::new("assets/textures/maps/cave/wall.png"));
	textures.insert("ceiling".into(), Texture::new("assets/textures/maps/cave/ceiling.png"));
	textures.insert("cube".into(), Texture::new("assets/textures/maps/cave/cube.png"));
	textures.insert("lowBar".into(), Texture::new("assets/textures/maps/cave/lowBar.png"));
	textures.insert("tallWall".into(), Texture::new("assets/textures/maps/cave/tallWall.png"));
	textures.insert("highBar".into(), Texture::new("assets/textures/maps/cave/highBar.png"));

    let character_mesh = Mesh::cube(Mesh::PLAYER_COLOR);
	let mut game_state = GameState::Menu;
	let mut previous_state= GameState::Menu;
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
		record: false,
		current_skin: Skins::Red("red".into()),
		current_map: Maps::Cave("cave".into()),
		textures,
		unlocked_maps: HashMap::from([
			("cave".into(), true),
			("temple".into(), false),
		]),
		unlocked_skins: HashMap::from([
			("red".into(), true),
			("troll".into(), false),
			("dirt".into(), false),
			("stone".into(), false),
			("diamond".into(), false),
			("emerald".into(), false),
			("arcane".into(), false),
		]),
		quest_progress: HashMap::from([
			("highScore".into(), 0),
			("caveScore".into(), 0),
			("templeScore".into(), 0),
			("deaths".into(), 0),
			("caveGames".into(), 0),
		]),
    };
	if let Ok(save_data) = load_progress() {
        world.unlocked_maps = save_data.unlocked_maps;
        world.unlocked_skins = save_data.unlocked_skins;
        world.quest_progress = save_data.quest_progress;
		if world.current_map != save_data.current_map {
			world.current_map = save_data.current_map;
			world.change_map();
		}
		if world.current_skin != save_data.current_skin {
			world.current_skin = save_data.current_skin;
			world.change_skin();
		}
    }

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
				_ => handle_keys(&mut window, event, &mut game_state, &mut character, &mut world, &glfw, &previous_state),
			}
        }

		match game_state {
			GameState::Menu => {
				unsafe { world.menu.render(&ui_shader, &text_shader, &world.textures["font"]); }
				if world.mouse_clicked {
					match world.menu.handle_click(world.mouse_x, world.mouse_y) {
						MenuAction::Play => new_game(&mut game_state, &mut character, &mut world, &glfw),
						MenuAction::MapSelect => game_state = GameState::MapSelect,
						MenuAction::SkinSelect => game_state = GameState::SkinSelect,
						MenuAction::Quit => window.set_should_close(true),
						MenuAction::None => {}
					}
					world.mouse_clicked = false;
				}
			},
			GameState::MapSelect => {
				let map_select = MapSelect::new(SCREEN_WIDTH, SCREEN_HEIGHT, &world);
				unsafe { map_select.render(&ui_shader, &text_shader, &world.current_map, &world.textures["font"]); }
				if world.mouse_clicked {
					match map_select.handle_click(world.mouse_x, world.mouse_y) {
						MapAction::SelectMap(map) => {
							world.current_map = map;
							world.change_map();
						}
						MapAction::ShowMessage(msg) => {
							previous_state = GameState::MapSelect;
							game_state = GameState::ShowMessage(msg);
						}
						MapAction::Back => game_state = GameState::Menu,
						_ => {}
					}
					world.mouse_clicked = false;
				}
			}
			GameState::SkinSelect => {
				let skin_select = SkinSelect::new(SCREEN_WIDTH, SCREEN_HEIGHT, &world);
				unsafe { skin_select.render(&ui_shader, &text_shader, &world.current_skin, &world.textures["font"]); }
				if world.mouse_clicked {
					match skin_select.handle_click(world.mouse_x, world.mouse_y) {
						SkinAction::SelectSkin(skin) => {
							world.current_skin = skin;
							world.change_skin();
						}
						SkinAction::ShowMessage(msg) => {
							previous_state = GameState::SkinSelect;
							game_state = GameState::ShowMessage(msg);
						}
						SkinAction::Back => game_state = GameState::Menu,
						_ => {}
					}
					world.mouse_clicked = false;
				}
			}
			GameState::ShowMessage(ref msg) => {
				unsafe {
					render_message(&msg, &ui_shader, &text_shader, world.screen_width, world.screen_height, &world.textures["font"]);
					if world.mouse_clicked {
						game_state = previous_state.clone();
						world.mouse_clicked = false;
					}
				}
			}
			GameState::Playing => {
				let current_time: f64 = glfw.get_time();
				let adjusted_time: f64 = current_time - world.total_pause_time;
				let delta_time: f32 = (adjusted_time - world.last_frame_time) as f32;
				world.last_frame_time = adjusted_time;
				character.update(delta_time);
				play(&mut world, &mut character, &mut game_state, &game_shader, &character_mesh, &text_shader, delta_time);
			}
			GameState::Paused => {
				unsafe { world.pause.render(&ui_shader, &text_shader, &world.textures["font"]); }
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
				unsafe {
					world.game_over.render(&ui_shader, &text_shader, *world.quest_progress.get("highScore").unwrap_or(&0), world.record, &world.textures["font"]);
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
	if let Err(e) = save_progress(&extract_save_data(&world)) {
		eprintln!("Error saving game progress: {}", e);
	}
}