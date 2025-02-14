use nalgebra::{Matrix4, Vector3};

use crate::math::{scaling, translation, orthographic};
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;

pub enum MenuAction {
    Play,
    Quit,
    None,
}

pub struct Button {
    mesh: Mesh,
    text_mesh: Mesh,
    position: (f32, f32),
    size: (f32, f32),
    color: Vector3<f32>,
}

pub struct Menu {
    buttons: Vec<Button>,
    ui_projection: Matrix4<f32>,
}

impl Menu {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let play_button = Button {
            mesh: Mesh::quad_2d(),
			text_mesh: Mesh::text("PLAY"),
            position: (screen_width / 2.0 - 200.0, screen_height / 2.0),
            size: (400.0, 100.0),
            color: Vector3::new(0.2, 1.0, 0.2),
        };

        let quit_button = Button {
            mesh: Mesh::quad_2d(),
			text_mesh: Mesh::text("EXIT"),
            position: (screen_width / 2.0 - 200.0, screen_height / 2.0 - 150.0),
            size: (400.0, 100.0),
            color: Vector3::new(1.0, 0.2, 0.2),
        };

        Menu {
            buttons: vec![play_button, quit_button],
            ui_projection: orthographic(0.0, screen_width, 0.0, screen_height, -1.0, 1.0),
        }
    }

    pub unsafe fn render(&self, shader: &Shader, text_shader: &Shader) {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);//gl::ClearColor(0.2, 0.4, 0.8, 1.0);
		gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

		shader.use_program();
        shader.set_mat4("projection", &self.ui_projection);

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
			text_shader.set_vec3("textColor", &Vector3::new(0.0, 0.0, 0.0));
			let font = Texture::new("assets/fonts/ChrustyRock.png");
			font.bind(0);
			let text_scale = 50.0;
			let text_width = button.text_mesh.indices_count as f32 / 6.0 * text_scale * 0.7;
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

    pub fn handle_click(&self, mouse_x: f32, mouse_y: f32) -> MenuAction {
        for (i, button) in self.buttons.iter().enumerate() {
            if mouse_x >= button.position.0 &&
                mouse_x <= button.position.0 + button.size.0 &&
                mouse_y >= button.position.1 &&
                mouse_y <= button.position.1 + button.size.1 
            {
                return match i {
                    0 => MenuAction::Play,
                    1 => MenuAction::Quit,
                    _ => MenuAction::None,
                };
            }
        }
        MenuAction::None
    }
}