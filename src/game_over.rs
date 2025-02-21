use nalgebra::{Matrix4, Vector3};
use crate::menu::Button;
use crate::math::{translation, scaling, orthographic};
use crate::shader::Shader;
use crate::mesh::Mesh;
use crate::texture::Texture;

pub enum GameOverAction {
	NewGame,
	Quit,
	None,
}

pub struct GameOver {
	pub buttons: Vec<Button>,
	pub ui_projection: Matrix4<f32>,
	pub screen_width: f32,
	pub screen_height: f32,
}

impl GameOver {
	pub fn new(screen_width: f32, screen_height: f32) -> Self {
		let resume_button = Button {
			mesh: Mesh::quad_2d(),
			text_mesh: Mesh::text("RETRY"),
			position: (screen_width / 2.0 - 150.0, screen_height / 2.0),
			size: (300.0, 80.0),
			color: Vector3::new(0.3, 0.8, 0.3),
		};
		let quit_button = Button {
			mesh: Mesh::quad_2d(),
			text_mesh: Mesh::text("QUIT"),
			position: (screen_width / 2.0 - 150.0, screen_height / 2.0 - 130.0),
			size: (300.0, 80.0),
			color: Vector3::new(0.9, 0.2, 0.2),
		};
	
		let buttons = vec![resume_button, quit_button];
		let ui_projection = orthographic(0.0, screen_width, 0.0, screen_height, -1.0, 1.0);
		GameOver {
			buttons,
			ui_projection,
			screen_width,
			screen_height
		}
	}
	
	pub unsafe fn render(&self, shader: &Shader, text_shader: &Shader, high_score: i32, record: bool) {
		gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

		// Title text
		text_shader.use_program();
		text_shader.set_mat4("projection", &self.ui_projection);
		text_shader.set_vec3("textColor", &Vector3::new(1.0, 0.2, 0.2));
	
		let font = Texture::new("assets/fonts/MinecraftRegular.png");
		font.bind(0);
	
		let text_scale = 60.0;
		let mut text_mesh = Mesh::text("GAME OVER");
		let text_width = text_mesh.indices_count as f32 / 6.0 * text_scale * 0.8;
		let x = self.screen_width / 2.0 - text_width / 2.0;
		let y = self.screen_height - 110.0;
	
		let text_model = translation(x, y, 0.0) * scaling(text_scale, text_scale, 1.0);
		text_shader.set_mat4("model", &text_model);
		text_mesh.draw();

        text_mesh = if record {
            Mesh::text("NEW HIGH SCORE!")
        } else {
            Mesh::text(&format!("HIGH SCORE: {}m", high_score))
        };

		let font = Texture::new("assets/fonts/MinecraftRegular.png");
		font.bind(0);

        let score_scale = 40.0;
        let score_width = text_mesh.indices_count as f32 / 6.0 * score_scale * 0.8;
        let score_model = translation(
            self.screen_width / 2.0 - score_width / 2.0,
            y - 65.0,
            0.0
        ) * scaling(score_scale, score_scale, 1.0);

        text_shader.set_mat4("model", &score_model);
        if record {
			text_shader.set_vec3("textColor", &Vector3::new(0.9, 0.9, 0.0))
		} else {
			text_shader.set_vec3("textColor", &Vector3::new(0.8, 0.8, 0.8))
		}
        text_mesh.draw();

        for button in &self.buttons {
			// Button background
			shader.use_program();
			shader.set_mat4("projection", &self.ui_projection);
			let model = translation(button.position.0, button.position.1, 0.0)
				* scaling(button.size.0, button.size.1, 1.0);
			shader.set_mat4("model", &model);
			shader.set_vec3("color", &button.color);
			button.mesh.draw();
			
			// Button text
			text_shader.use_program();
			text_shader.set_mat4("projection", &self.ui_projection);
			text_shader.set_vec3("textColor", &Vector3::new(0.1, 0.0, 0.0));
			let font = Texture::new("assets/fonts/MinecraftRegular.png");
			font.bind(0);
			let text_scale = 50.0;
			let text_width = button.text_mesh.indices_count as f32 / 6.0 * text_scale * 0.8;
			let text_model = translation(
				button.position.0 + button.size.0 / 2.0 - text_width / 2.0,
				button.position.1 + button.size.1 / 2.0 - text_scale / 2.0,
				0.0
			) * scaling(text_scale, text_scale, 1.0);
			text_shader.set_mat4("model", &text_model);
			button.text_mesh.draw();
		}
        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
	}

	pub fn handle_click(&self, mouse_x: f32, mouse_y: f32) -> GameOverAction {
		for (i, button) in self.buttons.iter().enumerate() {
			if mouse_x >= button.position.0 &&
				mouse_x <= button.position.0 + button.size.0 &&
				mouse_y >= button.position.1 &&
				mouse_y <= button.position.1 + button.size.1
			{
				return match i {
					0 => GameOverAction::NewGame,
					1 => GameOverAction::Quit,
					_ => GameOverAction::None,
				};
			}
		}
		GameOverAction::None
	}
}	