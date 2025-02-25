mod audio;
mod character;
mod controls;
mod game;
mod game_over;
mod level;
mod map_select;
mod math;
mod menu;
mod mesh;
mod pause;
mod save_data;
mod shader;
mod skin_select;
mod texture;

use crate::audio::AudioSystem;
use crate::controls::handle_keys;
use crate::game::{new_game, play};
use crate::game_over::{GameOver, GameOverAction};
use crate::level::LevelGenerator;
use crate::map_select::{MapAction, MapSelect, Maps};
use crate::menu::{render_message, Menu, MenuAction};
use crate::mesh::Mesh;
use crate::pause::{Pause, PauseAction};
use crate::save_data::{extract_save_data, load_progress, save_progress};
use crate::skin_select::{SkinAction, SkinSelect, Skins};
use crate::texture::Texture;
use glfw::{Action, Context, MouseButton, WindowEvent};
use std::collections::HashMap;

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
    current_music: Option<String>,
    textures: HashMap<String, Texture>,
    audio: AudioSystem,
    unlocked_maps: HashMap<String, bool>,
    unlocked_skins: HashMap<String, bool>,
    quest_progress: HashMap<String, i32>,
}

impl WorldState {
    fn change_map(&mut self) {
        let map_path = match &self.current_map {
            Maps::Campus(path) | Maps::Cave(path) | Maps::Temple(path) => {
                format!("assets/textures/maps/{}", path)
            }
            Maps::None => String::from("assets/textures/maps/campus.png"),
        };

        self.textures.insert(
            "floor".into(),
            Texture::new(&format!("{}/floor.png", map_path)),
        );
        self.textures.insert(
            "wall".into(),
            Texture::new(&format!("{}/wall.png", map_path)),
        );
        self.textures.insert(
            "ceiling".into(),
            Texture::new(&format!("{}/ceiling.png", map_path)),
        );
        self.textures.insert(
            "cube".into(),
            Texture::new(&format!("{}/cube.png", map_path)),
        );
        self.textures.insert(
            "lowBar".into(),
            Texture::new(&format!("{}/lowBar.png", map_path)),
        );
        self.textures.insert(
            "tallWall".into(),
            Texture::new(&format!("{}/tallWall.png", map_path)),
        );
        self.textures.insert(
            "highBar".into(),
            Texture::new(&format!("{}/highBar.png", map_path)),
        );
    }

    fn change_skin(&mut self) {
        let skin_path = match &self.current_skin {
            Skins::Red(path)
            | Skins::Troll(path)
            | Skins::Dirt(path)
            | Skins::Stone(path)
            | Skins::Diamond(path)
            | Skins::Emerald(path)
            | Skins::Arcane(path)
            | Skins::Jumper(path) => format!("assets/textures/skins/{}", path),
            Skins::None => String::from("assets/textures/skins/red.png"),
        };

        self.textures
            .insert("skin".into(), Texture::new(&format!("{}.png", skin_path)));
    }
}

fn main() {
    const SCREEN_WIDTH: f32 = 1024.0;
    const SCREEN_HEIGHT: f32 = 768.0;
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            "42run",
            glfw::WindowMode::Windowed,
        )
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

    let game_shader = shader::Shader::new("shaders/vertex/game.glsl", "shaders/fragment/game.glsl")
        .expect("Failed to load shaders");
    let ui_shader = shader::Shader::new("shaders/vertex/ui.glsl", "shaders/fragment/ui.glsl")
        .expect("Failed to load UI shaders");
    let text_shader = shader::Shader::new("shaders/vertex/text.glsl", "shaders/fragment/text.glsl")
        .expect("Failed to load text shaders");

    let mut textures = HashMap::new();
    textures.insert(
        "font".into(),
        Texture::new("assets/fonts/MinecraftRegular.png"),
    );
    textures.insert("skin".into(), Texture::new("assets/textures/skins/red.png"));
    textures.insert(
        "floor".into(),
        Texture::new("assets/textures/maps/campus/floor.png"),
    );
    textures.insert(
        "wall".into(),
        Texture::new("assets/textures/maps/campus/wall.png"),
    );
    textures.insert(
        "ceiling".into(),
        Texture::new("assets/textures/maps/campus/ceiling.png"),
    );
    textures.insert(
        "cube".into(),
        Texture::new("assets/textures/maps/campus/cube.png"),
    );
    textures.insert(
        "lowBar".into(),
        Texture::new("assets/textures/maps/campus/lowBar.png"),
    );
    textures.insert(
        "tallWall".into(),
        Texture::new("assets/textures/maps/campus/tallWall.png"),
    );
    textures.insert(
        "highBar".into(),
        Texture::new("assets/textures/maps/campus/highBar.png"),
    );

    let mut audio = AudioSystem::new();
    audio.load_sound("jump", "assets/sounds/jump.wav");
    audio.load_sound("slide", "assets/sounds/slide.wav");
    audio.load_sound("collision1", "assets/sounds/diarrhea.wav");
    audio.load_sound("collision2", "assets/sounds/explosion.wav");
    audio.load_sound("button1", "assets/sounds/button1.wav");
    audio.load_sound("button2", "assets/sounds/button2.wav");
    audio.music_volume(0.4);
    audio.sound_volume(0.4);

    let character_mesh = Mesh::cube(Mesh::PLAYER_COLOR);
    let mut game_state = GameState::Menu;
    let mut previous_state = GameState::Menu;
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
        current_map: Maps::Campus("campus".into()),
        current_music: None,
        textures,
        audio,
        unlocked_maps: HashMap::from([
            ("campus".into(), true),
            ("cave".into(), false),
            ("temple".into(), false),
        ]),
        unlocked_skins: HashMap::from([
            ("red".into(), true),
            ("jumper".into(), false),
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
            ("jumps".into(), 0),
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
    let mut map_select: MapSelect;
    let mut skin_select: SkinSelect;

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) };
                    world.screen_width = width as f32;
                    world.screen_height = height as f32;
                    world.menu = Menu::new(world.screen_width, world.screen_height);
                    world.pause = Pause::new(world.screen_width, world.screen_height);
                    world.game_over = GameOver::new(world.screen_width, world.screen_height);
                }
                WindowEvent::CursorPos(x, y) => {
                    world.mouse_x = x as f32;
                    world.mouse_y = world.screen_height - y as f32;
                }
                WindowEvent::MouseButton(button, action, _) => {
                    if button == MouseButton::Left && action == Action::Press {
                        world.mouse_clicked = true;
                    }
                }
                _ => handle_keys(
                    &mut window,
                    event,
                    &mut game_state,
                    &mut character,
                    &mut world,
                    &glfw,
                    &previous_state,
                ),
            }
        }

        match game_state {
            GameState::Menu => {
                if world.current_music != Some("menu".to_string()) {
                    world.audio.play_music("assets/music/megalovania.wav");
                    world.current_music = Some("menu".to_string());
                }
                unsafe {
                    world
                        .menu
                        .render(&ui_shader, &text_shader, &world.textures["font"]);
                }
                if world.mouse_clicked {
                    match world
                        .menu
                        .handle_click(world.mouse_x, world.mouse_y, &world.audio)
                    {
                        MenuAction::Play => {
                            new_game(&mut game_state, &mut character, &mut world, &glfw)
                        }
                        MenuAction::MapSelect => game_state = GameState::MapSelect,
                        MenuAction::SkinSelect => game_state = GameState::SkinSelect,
                        MenuAction::Quit => window.set_should_close(true),
                        MenuAction::None => {}
                    }
                    world.mouse_clicked = false;
                }
            }
            GameState::MapSelect => {
                map_select = MapSelect::new(
                    world.screen_width,
                    world.screen_height,
                    &world.unlocked_maps,
                );
                unsafe {
                    map_select.render(
                        &ui_shader,
                        &text_shader,
                        &world.current_map,
                        &world.textures["font"],
                    );
                }
                if world.mouse_clicked {
                    match map_select.handle_click(
                        world.mouse_x,
                        world.mouse_y,
                        &world.audio,
                        &world.current_map,
                    ) {
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
                skin_select = SkinSelect::new(
                    world.screen_width,
                    world.screen_height,
                    &world.unlocked_skins,
                );
                unsafe {
                    skin_select.render(
                        &ui_shader,
                        &text_shader,
                        &world.current_skin,
                        &world.textures["font"],
                    );
                }
                if world.mouse_clicked {
                    match skin_select.handle_click(
                        world.mouse_x,
                        world.mouse_y,
                        &world.audio,
                        &world.current_skin,
                    ) {
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
            GameState::ShowMessage(ref msg) => unsafe {
                render_message(
                    &msg,
                    &ui_shader,
                    &text_shader,
                    world.screen_width,
                    world.screen_height,
                    &world.textures["font"],
                );
                if world.mouse_clicked {
                    game_state = previous_state.clone();
                    world.mouse_clicked = false;
                }
            },
            GameState::Playing => {
                let map_music = match world.current_map {
                    Maps::Campus(_) => "assets/music/death_by_glamour.wav",
                    Maps::Cave(_) => "assets/music/heartache.wav",
                    Maps::Temple(_) => "assets/music/spear_of_justice.wav",
                    _ => "assets/music/megalovania.wav",
                };

                if world.current_music.as_deref() != Some(map_music) {
                    world.audio.play_music(map_music);
                    world.current_music = Some(map_music.to_string());
                }

                let current_time: f64 = glfw.get_time();
                let adjusted_time: f64 = current_time - world.total_pause_time;
                let delta_time: f32 = (adjusted_time - world.last_frame_time) as f32;
                world.last_frame_time = adjusted_time;
                character.update(delta_time);
                play(
                    &mut world,
                    &mut character,
                    &mut game_state,
                    &game_shader,
                    &character_mesh,
                    &text_shader,
                    delta_time,
                );
            }
            GameState::Paused => {
                world.audio.pause_music();
                unsafe {
                    world
                        .pause
                        .render(&ui_shader, &text_shader, &world.textures["font"]);
                }
                if world.mouse_clicked {
                    match world
                        .pause
                        .handle_click(world.mouse_x, world.mouse_y, &world.audio)
                    {
                        PauseAction::Resume => {
                            world.audio.resume_music();
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
                    world.game_over.render(
                        &ui_shader,
                        &text_shader,
                        *world.quest_progress.get("highScore").unwrap_or(&0),
                        world.record,
                        &world.textures["font"],
                    );
                }
                if world.mouse_clicked {
                    match world
                        .game_over
                        .handle_click(world.mouse_x, world.mouse_y, &world.audio)
                    {
                        GameOverAction::NewGame => {
                            new_game(&mut game_state, &mut character, &mut world, &glfw)
                        }
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
