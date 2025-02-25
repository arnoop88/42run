use crate::audio::AudioSystem;
use crate::math::{orthographic, scaling, translation};
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;
use nalgebra::{Matrix4, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    Jumper(String),
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
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        unlocked_skins: &HashMap<String, bool>,
    ) -> Self {
        let buttons = vec![
            SkinButton {
                id: Skins::Red("red".into()),
                unlocked: true,
                unlock_requirement: "".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("RED"),
                position: Self::button_position(0, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::Jumper("jumper".into()),
                unlocked: unlocked_skins["jumper"],
                unlock_requirement: "Jump 500 times".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("JUMPER"),
                position: Self::button_position(1, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::Troll("trollFace".into()),
                unlocked: unlocked_skins["troll"],
                unlock_requirement: "Die 100 times".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("TROLL"),
                position: Self::button_position(2, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::Dirt("dirt".into()),
                unlocked: unlocked_skins["dirt"],
                unlock_requirement: "Reach 300m in cave".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("DIRT"),
                position: Self::button_position(3, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::Stone("chiseledStone".into()),
                unlocked: unlocked_skins["stone"],
                unlock_requirement: "Reach 300m in temple".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("STONE"),
                position: Self::button_position(4, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::Diamond("diamondBlock".into()),
                unlocked: unlocked_skins["diamond"],
                unlock_requirement: "Reach 500m in cave".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("DIAMOND"),
                position: Self::button_position(5, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::Emerald("emeraldBlock".into()),
                unlocked: unlocked_skins["emerald"],
                unlock_requirement: "Reach 500m in temple".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("EMERALD"),
                position: Self::button_position(6, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::Arcane("arcane".into()),
                unlocked: unlocked_skins["arcane"],
                unlock_requirement: "Reach 1000m in any map".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("ARCANE"),
                position: Self::button_position(7, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            SkinButton {
                id: Skins::None,
                unlocked: true,
                unlock_requirement: "".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("BACK"),
                position: Self::button_position(9, screen_width, screen_height),
                size: (300.0, 80.0),
                color: Vector3::new(0.9, 0.6, 0.0),
            },
        ];

        SkinSelect {
            buttons,
            ui_projection: orthographic(0.0, screen_width, 0.0, screen_height, -1.0, 1.0),
            screen_width,
            screen_height,
        }
    }

    pub unsafe fn render(
        &self,
        shader: &Shader,
        text_shader: &Shader,
        skin: &Skins,
        font: &Texture,
    ) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        // Title text
        text_shader.use_program();
        text_shader.set_mat4("projection", &self.ui_projection);
        text_shader.set_vec3("textColor", &Vector3::new(0.5, 0.3, 0.7));
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
            font.bind(0);
            let text_scale = 50.0;
            let text_width = button.text_mesh.indices_count as f32 / 6.0 * text_scale * 0.8;
            let text_model = translation(
                button.position.0 + button.size.0 / 2.0 - text_width / 2.0,
                button.position.1 + button.size.1 / 2.0 - text_scale / 2.0,
                0.0,
            ) * scaling(text_scale, text_scale, 1.0);
            text_shader.set_mat4("model", &text_model);
            button.text_mesh.draw();
        }
    }

    fn button_position(index: usize, screen_width: f32, screen_height: f32) -> (f32, f32) {
        const BUTTON_WIDTH: f32 = 300.0;
        const BUTTON_HEIGHT: f32 = 80.0;
        const HORIZONTAL_SPACING: f32 = 20.0;
        const VERTICAL_SPACING: f32 = 20.0;
        const COLUMNS: usize = 3;

        match index {
            9 => (screen_width / 2.0 - BUTTON_WIDTH / 2.0, 50.0),
            _ => {
                let row = index / COLUMNS;
                let col = index % COLUMNS;

                let x_start = (screen_width
                    - (COLUMNS as f32 * BUTTON_WIDTH
                        + (COLUMNS as f32 - 1.0) * HORIZONTAL_SPACING))
                    / 2.0;

                let y_start = screen_height / 2.0 + 50.0;

                (
                    x_start + col as f32 * (BUTTON_WIDTH + HORIZONTAL_SPACING),
                    y_start - row as f32 * (BUTTON_HEIGHT + VERTICAL_SPACING),
                )
            }
        }
    }

    pub fn handle_click(
        &self,
        mouse_x: f32,
        mouse_y: f32,
        audio: &AudioSystem,
        current: &Skins,
    ) -> SkinAction {
        for (i, button) in self.buttons.iter().enumerate() {
            if mouse_x >= button.position.0
                && mouse_x <= button.position.0 + button.size.0
                && mouse_y >= button.position.1
                && mouse_y <= button.position.1 + button.size.1
            {
                if !button.unlocked {
                    return SkinAction::ShowMessage(button.unlock_requirement.clone());
                }
                if button.id == Skins::None {
                    audio.play_sound("button1");
                } else if button.id != *current {
                    audio.play_sound("button2");
                }
                return match i {
                    0..=7 => SkinAction::SelectSkin(button.id.clone()),
                    8 => SkinAction::Back,
                    _ => SkinAction::None,
                };
            }
        }
        SkinAction::None
    }
}
