use glfw::{Action, Key, WindowEvent};
use crate::GameState;
use crate::character::Character;
use crate::WorldState;
use crate::new_game;

pub fn handle_keys(window: &mut glfw::Window, event: WindowEvent, game_state: &mut GameState, character: &mut Character, world: &mut WorldState, glfw: &glfw::Glfw, previous_state: &GameState) {
    match event {
        glfw::WindowEvent::Key(key, _, action, _) => {
            match game_state {
                GameState::Playing => match key {
                    Key::Escape | Key::Q if action == Action::Press => {
                        *game_state = GameState::Paused;
                        world.pause_start_time = glfw.get_time();
						character.move_down(false);
                    }
                    Key::Left | Key::A if action == Action::Press => character.move_left(&world.audio),
                    Key::Right | Key::D if action == Action::Press => character.move_right(&world.audio),
                    Key::Space | Key::Up | Key::W if action == Action::Press => {
						character.jump(&world.audio);
						if !world.unlocked_skins["jumper"] {
							let jumps = world.quest_progress.entry("jumps".into()).or_insert(0);
							*jumps += 1;
							if *jumps >= 500 {
								world.unlocked_skins.insert("jumper".into(), true);
							}
						}
					}
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
					Key::Escape if action == Action::Press => {
						world.audio.play_sound("button1");
						window.set_should_close(true);
					}
					Key::Enter if action == Action::Press => {
						world.audio.play_sound("button1");
						new_game(game_state, character, world, glfw);
					}
                    _ => {}
                },
				GameState::MapSelect | GameState::SkinSelect => match key {
					Key::Escape if action == Action::Press => {
						world.audio.play_sound("button1");
						*game_state = GameState::Menu;
					}
                    _ => {}
                },
				GameState::ShowMessage(_) => match key {
					Key::Escape if action == Action::Press => *game_state = previous_state.clone(),
					_ => {}
				}
                GameState::Paused => match key {
					Key::Escape | Key::Q if action == Action::Press => {
						world.audio.play_sound("button1");
						*game_state = GameState::Menu;
					}
                    Key::Enter | Key::R if action == Action::Press => {
						world.audio.play_sound("button1");
						world.audio.resume_music();
						*game_state = GameState::Playing;
						world.total_pause_time += glfw.get_time() - world.pause_start_time;
					}
                    _ => {}
                },
                GameState::GameOver => match key {
                    Key::Escape | Key::Q if action == Action::Press => {
						world.audio.play_sound("button1");
						*game_state = GameState::Menu;
					}
					Key::Enter | Key::R if action == Action::Press => {
						world.audio.play_sound("button1");
						new_game(game_state, character, world, glfw);
					}
                    _ => {}
                },
            }
        }
        _ => {}
    }
}