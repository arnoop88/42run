use nalgebra::{Matrix4, Vector3};
use crate::math::{scaling, translation, orthographic};
use crate::menu::Button;
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::WorldState;

pub enum SkinAction {
	SelectSkin(String),
	ShowMessage(String),
	Back,
	None,
}
pub struct SkinSelect {
    buttons: Vec<Button>,
    ui_projection: Matrix4<f32>,
    screen_width: f32,
    screen_height: f32,
}

impl SkinSelect {
    pub fn new(screen_width: f32, screen_height: f32, world: &WorldState) -> Self {
        let buttons = vec![
            Button {
				id: "red".into(),
				unlocked: true,
				unlock_requirement: "".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("RED"),
                position: (screen_width / 4.0 - 210.0, screen_height / 2.0 + 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            Button {
				id: "trollFace".into(),
				unlocked: *world.unlocked_skins.get("trollFace").unwrap_or(&false),
				unlock_requirement: "Die 100 times".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("TROLL"),
                position: (screen_width / 2.0  - 150.0, screen_height / 2.0 + 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			Button {
				id: "dirt".into(),
				unlocked: *world.unlocked_skins.get("dirt").unwrap_or(&false),
				unlock_requirement: "Reach 300m in cave".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("DIRT"),
                position: (screen_width * 0.75 - 90.0, screen_height / 2.0 + 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			Button {
				id: "chiseledStone".into(),
				unlocked: *world.unlocked_skins.get("chiseledStone").unwrap_or(&false),
				unlock_requirement: "Reach 300m in temple".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("STONE"),
                position: (screen_width / 4.0 - 210.0, screen_height / 2.0 - 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			Button {
				id: "diamondBlock".into(),
				unlocked: *world.unlocked_skins.get("diamondBlock").unwrap_or(&false),
				unlock_requirement: "Reach 500m in cave".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("DIAMOND"),
                position: (screen_width / 2.0 - 150.0, screen_height / 2.0 - 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            Button {
				id: "emeraldBlock".into(),
				unlocked: *world.unlocked_skins.get("emeraldBlock").unwrap_or(&false),
				unlock_requirement: "Reach 500m in temple".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("EMERALD"),
                position: (screen_width * 0.75  - 90.0, screen_height / 2.0 - 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			Button {
				id: "arcane".into(),
				unlocked: *world.unlocked_skins.get("arcane").unwrap_or(&false),
				unlock_requirement: "Reach 1000m in any map".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("ARCANE"),
                position: (screen_width / 4.0 - 210.0, screen_height / 2.0 - 150.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			Button {
				id: "".into(),
				unlocked: true,
				unlock_requirement: "".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("BACK"),
                position: (screen_width / 2.0 - 150.0, 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.9, 0.6, 0.0),
            }
        ];

        SkinSelect {
            buttons,
            ui_projection: orthographic(0.0, screen_width, 0.0, screen_height, -1.0, 1.0),
            screen_width,
            screen_height,
        }
    }

    pub unsafe fn render(&self, shader: &Shader, text_shader: &Shader, skin: &str) {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        // Title text
        text_shader.use_program();
        text_shader.set_mat4("projection", &self.ui_projection);
        text_shader.set_vec3("textColor", &Vector3::new(0.4, 0.6, 1.0));
    
        let font = Texture::new("assets/fonts/MinecraftRegular.png");
        font.bind(0);
    
        let text_scale = 60.0;
        let text_mesh = Mesh::text("SELECT SKIN");
        let text_width = text_mesh.indices_count as f32 / 6.0 * text_scale * 0.8;
        let x = self.screen_width / 2.0 - text_width / 2.0;
        let y = self.screen_height / 5.0 * 4.0;
    
        let text_model = translation(x, y, 0.0) * scaling(text_scale, text_scale, 1.0);
        text_shader.set_mat4("model", &text_model);
        text_mesh.draw();

        for button in &self.buttons {
            let color = if button.id == skin {
				Vector3::new(0.4, 0.6, 1.0)
			} else {
                button.color
            };
			
			shader.use_program();
			shader.set_mat4("projection", &self.ui_projection);
			let model = translation(button.position.0, button.position.1, 0.0)
				* scaling(button.size.0, button.size.1, 1.0);
			shader.set_mat4("model", &model);
			shader.set_vec3("color", &color);
			button.mesh.draw();
			
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
    }

    pub fn handle_click(&self, mouse_x: f32, mouse_y: f32) -> SkinAction {
        for (i, button) in self.buttons.iter().enumerate() {
            if mouse_x >= button.position.0 &&
                mouse_x <= button.position.0 + button.size.0 &&
                mouse_y >= button.position.1 &&
                mouse_y <= button.position.1 + button.size.1 
            {
				if !button.unlocked {
					return SkinAction::ShowMessage(button.unlock_requirement.clone());
				}
                return match i {
                    0..=6 => SkinAction::SelectSkin(button.id.clone()),
                    7 => SkinAction::Back,
                    _ => SkinAction::None,
                };
            }
        }
        SkinAction::None
    }
}