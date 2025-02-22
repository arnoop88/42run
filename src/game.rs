use nalgebra::Vector3;
use crate::WorldState;
use crate::character::Character;
use crate::LevelGenerator;
use crate::GameState;
use crate::math;
use crate::Maps;
use crate::shader::Shader;
use crate::mesh::Mesh;
use crate::level::ObstacleType;

pub fn new_game(game_state: &mut GameState, character: &mut Character, world: &mut WorldState, glfw: &glfw::Glfw) {
	world.z = 0.0;
	world.speed = 20.0;
	*character = Character::new();
	world.level = LevelGenerator::new();
	*game_state = GameState::Playing;
	world.total_pause_time = 0.0;
    world.pause_start_time = 0.0;
    world.last_frame_time = glfw.get_time();
	world.record = false;
}

pub fn play(world: &mut WorldState, character: &mut Character, game_state: &mut GameState, game_shader: &Shader, character_mesh: &Mesh, text_shader: &Shader, delta_time: f32) {
	world.z += world.speed * delta_time;
    world.level.update(world.z);
	world.speed = (world.speed + 0.3 * delta_time).min(50.0);
    
    character.update(delta_time);
    
    let mut collision_detected = false;
    let player_aabb = character.get_aabb(world.z);

    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Camera setup
        let eye = Vector3::new(0.0, 3.0, -10.0);
        let target = Vector3::new(0.0, 1.5, 0.0);
        let view = math::look_at(eye, target, Vector3::new(0.0, 1.0, 0.0));
        let projection = math::perspective(45.0f32.to_radians(), world.screen_width /  world.screen_height, 0.1, 1000.0);
        
		game_shader.use_program();
        game_shader.set_mat4("view", &view);
        game_shader.set_mat4("projection", &projection);

        // Render level segments
        for segment in world.level.segments() {
            let segment_z = segment.position - world.z;
            if segment_z < -25.0 {
                continue;
            }

            // Platform rendering
            let model = math::translation(0.0, 0.0, segment_z);
            game_shader.set_mat4("model", &model);
			game_shader.set_int("texture_diffuse", 0);
			world.textures["floor"].bind(0);
            segment.platform.draw();

			// Left wall
			let wall_model = math::translation(3.0, 0.0, segment_z);
			game_shader.set_mat4("model", &wall_model);
			world.textures["wall"].bind(0);
			segment.wall.draw();

			// Right wall
			let wall_model = math::translation(-3.0, 0.0, segment_z);
			game_shader.set_mat4("model", &wall_model);
			segment.wall.draw();

			// Ceiling
			let ceiling_model = math::translation(0.0, 5.0, segment_z);
			game_shader.set_mat4("model", &ceiling_model);
			world.textures["ceiling"].bind(0);
			segment.platform.draw();

            // Obstacle handling
            for obstacle in &segment.obstacles {
                let obstacle_aabb = obstacle.get_aabb();
                if player_aabb.collides(&obstacle_aabb) {
                    collision_detected = true;
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
				match obstacle.obstacle_type {
					ObstacleType::Cube => world.textures["cube"].bind(0),
					ObstacleType::LowBar => world.textures["lowBar"].bind(0),
					ObstacleType::TallWall => world.textures["tallWall"].bind(0),
					ObstacleType::HighBar => world.textures["highBar"].bind(0),
				}
                obstacle.mesh.draw();
            }
        }

        // Character rendering
        let model = math::translation(
            character.position.x,
            character.position.y + 0.001,
            0.0
        ) * math::scaling(1.0, character.current_height, 1.0);
        game_shader.set_mat4("model", &model);
		world.textures["skin"].bind(0);
        character_mesh.draw();

		// Distance rendering
		gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        let distance_text = format!("{}m", world.z as i32 / 10);
        let text_mesh = Mesh::text(&distance_text);
        
        text_shader.use_program();
        let ui_projection = math::orthographic(0.0, world.screen_width, 0.0, world.screen_height, -1.0, 1.0);
        text_shader.set_mat4("projection", &ui_projection);
        world.textures["font"].bind(0);
        text_shader.set_vec3("textColor", &Vector3::new(0.9, 0.9, 0.9));

        let text_scale = 40.0;
        let text_model = math::translation(
            10.0,
            world.screen_height - 50.0,
            0.0
        ) * math::scaling(text_scale, text_scale, 1.0);
        
        text_shader.set_mat4("model", &text_model);
        text_mesh.draw();

        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
    }

    if collision_detected {
		let score = world.z as i32 / 10;
	
		// Update deaths and unlock troll skin
		if !world.unlocked_skins["troll"] {
			let deaths = world.quest_progress.entry("deaths".into()).or_insert(0);
			*deaths += 1;
			if *deaths >= 100 {
				world.unlocked_skins.insert("troll".into(), true);
			}
		}
	
		// Update highScore and unlock arcane skin
		if score > world.quest_progress["highScore"] {
			world.quest_progress.insert("highScore".into(), score);
			world.record = true;
			if !world.unlocked_skins["arcane"] && score >= 1000 {
				world.unlocked_skins.insert("arcane".into(), true);
			}
		}

		match world.current_map {
			Maps::Cave(..) => {
				// Update caveGames progress and unlock temple map
				if !world.unlocked_maps["temple"] {
					let cave_games = world.quest_progress.entry("caveGames".into()).or_insert(0);
					*cave_games += 1;
					if *cave_games >= 15 {
						world.unlocked_maps.insert("temple".into(), true);
					}
				}
				// Update caveScore and unlock cave skins
				if score > world.quest_progress["caveScore"] {
					world.quest_progress.insert("caveScore".into(), score);
					if !world.unlocked_skins["diamond"] {
						if !world.unlocked_skins["dirt"] && score >= 300 {
							world.unlocked_skins.insert("dirt".into(), true);
						}
						if score >= 500 {
							world.unlocked_skins.insert("diamond".into(), true);
						}
					}
				}
			}
			Maps::Temple(..) => {
				// Update templeScore and unlock temple skins
				if score > world.quest_progress["templeScore"] {
					world.quest_progress.insert("templeScore".into(), score);
					if !world.unlocked_skins["emerald"] {
						if !world.unlocked_skins["stone"] && score >= 300 {
							world.unlocked_skins.insert("stone".into(), true);
						}
						if score >= 500 {
							world.unlocked_skins.insert("emerald".into(), true);
						}
					}
				}
			}
			_ => {}
		}
		*game_state = GameState::GameOver;
	}
	
}