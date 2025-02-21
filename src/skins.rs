use nalgebra::{Matrix4, Vector3};
use serde::{Serialize, Deserialize};
use crate::math::{scaling, translation, orthographic};
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::WorldState;

pub struct SkinButton {
	pub id: Skins,
	pub unlocked: bool,
    pub unlock_requirement: String,
    pub mesh: Mesh,
    pub text_mesh: Mesh,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: Vector3<f32>,
}

pub enum SkinAction {
	SelectSkin(Skins),
	ShowMessage(String),
	Back,
	None,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Skins {
	Red(String),
	Troll(String),
	Dirt(String),
	Stone(String),
	Diamond(String),
	Emerald(String),
	Arcane(String),
	None,
}
pub struct SkinSelect {
    buttons: Vec<SkinButton>,
    ui_projection: Matrix4<f32>,
    screen_width: f32,
    screen_height: f32,
}

impl SkinSelect {
    pub fn new(screen_width: f32, screen_height: f32, world: &WorldState) -> Self {
        let buttons = vec![
            SkinButton {
				id: Skins::Red("red".into()),
				unlocked: true,
				unlock_requirement: "".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("RED"),
                position: (screen_width / 4.0 - 210.0, screen_height / 2.0 + 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
				id: Skins::Troll("trollFace".into()),
				unlocked: world.unlocked_skins["troll"],
				unlock_requirement: "Die 100 times".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("TROLL"),
                position: (screen_width / 2.0  - 150.0, screen_height / 2.0 + 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			SkinButton {
				id: Skins::Dirt("dirt".into()),
				unlocked:  world.unlocked_skins["dirt"],
				unlock_requirement: "Reach 300m in cave".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("DIRT"),
                position: (screen_width * 0.75 - 90.0, screen_height / 2.0 + 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			SkinButton {
				id: Skins::Stone("chiseledStone".into()),
				unlocked:  world.unlocked_skins["stone"],
				unlock_requirement: "Reach 300m in temple".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("STONE"),
                position: (screen_width / 4.0 - 210.0, screen_height / 2.0 - 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			SkinButton {
				id: Skins::Diamond("diamondBlock".into()),
				unlocked:  world.unlocked_skins["diamond"],
				unlock_requirement: "Reach 500m in cave".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("DIAMOND"),
                position: (screen_width / 2.0 - 150.0, screen_height / 2.0 - 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
				id: Skins::Emerald("emeraldBlock".into()),
				unlocked:  world.unlocked_skins["emerald"],
				unlock_requirement: "Reach 500m in temple".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("EMERALD"),
                position: (screen_width * 0.75  - 90.0, screen_height / 2.0 - 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			SkinButton {
				id: Skins::Arcane("arcane".into()),
				unlocked:  world.unlocked_skins["arcane"],
				unlock_requirement: "Reach 1000m in any map".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("ARCANE"),
                position: (screen_width / 4.0 - 210.0, screen_height / 2.0 - 150.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
			SkinButton {
				id: Skins::None,
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

    pub unsafe fn render(&self, shader: &Shader, text_shader: &Shader, skin: &Skins) {
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
            let color = if button.id == *skin {
				Vector3::new(0.9, 0.8, 0.4)
			} else if button.unlocked && button.id != Skins::None {
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