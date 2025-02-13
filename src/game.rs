use crate::WorldState;
use crate::character::Character;
use crate::LevelGenerator;
use crate::GameState;
use crate::math;

use nalgebra::Vector3;

pub fn new_game(game_state: &mut GameState, character: &mut Character, world: &mut WorldState, glfw: &glfw::Glfw) {
	world.z = 0.0;
	world.speed = 20.0;
	*character = Character::new();
	world.level = LevelGenerator::new();
	*game_state = GameState::Playing;
	world.total_pause_time = 0.0;
    world.pause_start_time = 0.0;
    world.last_frame_time = glfw.get_time();
}

pub fn play(world: &mut WorldState, character: &mut Character, game_state: &mut GameState, game_shader: &crate::shader::Shader, character_mesh: &crate::mesh::Mesh, delta_time: f32) {
    world.z += world.speed * delta_time;
    world.level.update(world.z);
    world.speed = (world.speed + 0.2 * delta_time).min(50.0);
    
    character.update(delta_time);
    
    let mut collision_detected = false;
    let player_aabb = character.get_aabb(world.z);

    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        game_shader.use_program();

        // Camera setup
        let eye = Vector3::new(0.0, 3.0, -10.0);
        let target = Vector3::new(0.0, 1.5, 0.0);
        let view = math::look_at(eye, target, Vector3::new(0.0, 1.0, 0.0));
        let projection = math::perspective(45.0f32.to_radians(), 800.0 / 600.0, 0.1, 1000.0);
        
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
            segment.platform.draw();

            // Obstacle handling
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

        // Character rendering
        let model = math::translation(
            character.position.x,
            character.position.y + 0.5,
            0.0
        );
        game_shader.set_mat4("model", &model);
        character_mesh.draw();
    }

    if collision_detected {
        *game_state = GameState::GameOver;
    }
}