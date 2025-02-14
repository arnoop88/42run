use glfw::{Action, Key, WindowEvent};

use crate::GameState;
use crate::character::Character;
use crate::WorldState;
use crate::new_game;

pub fn handle_keys(window: &mut glfw::Window, event: WindowEvent, game_state: &mut GameState, character: &mut Character, world: &mut WorldState, glfw: &glfw::Glfw) {
    match event {
        glfw::WindowEvent::Key(key, _, action, _) => {
            match game_state {
                GameState::Playing => match key {
                    Key::Escape if action == Action::Press => {
                        *game_state = GameState::Paused;
                        world.pause_start_time = glfw.get_time();
                    }
                    Key::Left | Key::A if action == Action::Press => character.move_left(),
                    Key::Right | Key::D if action == Action::Press => character.move_right(),
                    Key::Space | Key::Up | Key::W if action == Action::Press => character.jump(),
                    Key::Down | Key::S => {
						let is_pressed = match action {
							glfw::Action::Press | glfw::Action::Repeat => true,
							glfw::Action::Release => false
						};
						character.move_down(is_pressed);
					}
                    _ => {}
                },
                GameState::Menu => match key {
					Key::Escape if action == Action::Press => window.set_should_close(true),
					Key::Enter if action == Action::Press => new_game(game_state, character, world, glfw),
                    _ => {}
                },
                GameState::Paused => match key {
					Key::Escape if action == Action::Press => *game_state = GameState::Menu,
                    Key::Enter if action == Action::Press => {
						*game_state = GameState::Playing;
						world.total_pause_time += glfw.get_time() - world.pause_start_time;
					}
                    _ => {}
                },
                GameState::GameOver => match key {
                    Key::Escape if action == Action::Press => *game_state = GameState::Menu,
					Key::Enter if action == Action::Press => new_game(game_state, character, world, glfw),
                    _ => {}
                },
            }
        }
        _ => {}
    }
}