use glfw::{Action, Key, WindowEvent};

use crate::GameState;
use crate::character::Character;
use crate::WorldState;
use crate::new_game;

pub fn handle_keys(window: &mut glfw::Window, event: WindowEvent, game_state: &mut GameState, character: &mut Character, world: &mut WorldState, glfw: &glfw::Glfw) {
    match event {
        glfw::WindowEvent::Key(key, _, Action::Press, _) => {
            match game_state {
                GameState::Playing => match key {
					Key::Escape => {
						*game_state = GameState::Paused;
						world.pause_start_time = glfw.get_time();
					}
                    Key::Left | Key::A => character.move_left(),
                    Key::Right | Key::D => character.move_right(),
                    Key::Space | Key::Up | Key::W => character.jump(),
                    _ => {}
                },
                GameState::Menu => match key {
					Key::Escape => window.set_should_close(true),
					Key::Enter => new_game(game_state, character, world, glfw),
                    _ => {}
                },
                GameState::Paused => match key {
					Key::Escape => *game_state = GameState::Menu,
                    Key::Enter => {
						*game_state = GameState::Playing;
						world.total_pause_time += glfw.get_time() - world.pause_start_time;
					}
                    _ => {}
                },
                GameState::GameOver => match key {
                    Key::Escape => *game_state = GameState::Menu,
					Key::Enter => new_game(game_state, character, world, glfw),
                    _ => {}
                },
            }
        }
        _ => {}
    }
}