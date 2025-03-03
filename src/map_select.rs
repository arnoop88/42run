use crate::audio::AudioSystem;
use crate::math::{orthographic, scaling, translation};
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;
use nalgebra::{Matrix4, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MapButton {
    pub id: Maps,
    pub unlocked: bool,
    pub unlock_requirement: String,
    pub mesh: Mesh,
    pub text_mesh: Mesh,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: Vector3<f32>,
}

pub enum MapAction {
    SelectMap(Maps),
    ShowMessage(String),
    Back,
    None,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Maps {
    Campus(String),
    Cave(String),
    Temple(String),
    None,
}

pub struct MapSelect {
    buttons: Vec<MapButton>,
    ui_projection: Matrix4<f32>,
    screen_width: f32,
    screen_height: f32,
}

impl MapSelect {
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        unlocked_maps: &HashMap<String, bool>,
    ) -> Self {
        let buttons = vec![
            MapButton {
                id: Maps::Campus("campus".into()),
                unlocked: unlocked_maps["campus"],
                unlock_requirement: "".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("CAMPUS"),
                position: (screen_width / 2.0 - 150.0, screen_height / 2.0 + 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            MapButton {
                id: Maps::Cave("cave".into()),
                unlocked: unlocked_maps["cave"],
                unlock_requirement: "reach 100m".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("CAVE"),
                position: (screen_width / 2.0 - 150.0, screen_height / 2.0 - 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            MapButton {
                id: Maps::Temple("temple".into()),
                unlocked: unlocked_maps["temple"],
                unlock_requirement: "Play 15 games in cave".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("TEMPLE"),
                position: (screen_width / 2.0 - 150.0, screen_height / 2.0 - 150.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.4, 0.4, 0.4),
            },
            MapButton {
                id: Maps::None,
                unlocked: true,
                unlock_requirement: "".into(),
                mesh: Mesh::quad_2d(),
                text_mesh: Mesh::text("BACK"),
                position: (screen_width / 2.0 - 150.0, 50.0),
                size: (300.0, 80.0),
                color: Vector3::new(0.9, 0.6, 0.0),
            },
        ];

        MapSelect {
            buttons,
            ui_projection: orthographic(0.0, screen_width, 0.0, screen_height, -1.0, 1.0),
            screen_width,
            screen_height,
        }
    }

    pub unsafe fn render(&self, shader: &Shader, text_shader: &Shader, map: &Maps, font: &Texture) {
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
        let text_mesh = Mesh::text("SELECT MAP");
        let text_width = text_mesh.indices_count as f32 / 6.0 * text_scale * 0.8;
        let x = self.screen_width / 2.0 - text_width / 2.0;
        let y = self.screen_height / 5.0 * 4.0;

        let text_model = translation(x, y, 0.0) * scaling(text_scale, text_scale, 1.0);
        text_shader.set_mat4("model", &text_model);
        text_mesh.draw();

        for button in &self.buttons {
            let color = if button.id == *map {
                Vector3::new(0.9, 0.8, 0.4)
            } else if button.unlocked && button.id != Maps::None {
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

    pub fn handle_click(
        &self,
        mouse_x: f32,
        mouse_y: f32,
        audio: &AudioSystem,
        current: &Maps,
    ) -> MapAction {
        for (i, button) in self.buttons.iter().enumerate() {
            if mouse_x >= button.position.0
                && mouse_x <= button.position.0 + button.size.0
                && mouse_y >= button.position.1
                && mouse_y <= button.position.1 + button.size.1
            {
                if !button.unlocked {
                    return MapAction::ShowMessage(button.unlock_requirement.clone());
                }
                if button.id == Maps::None {
                    audio.play_sound("button1");
                } else if button.id != *current {
                    audio.play_sound("button2");
                }
                return match i {
                    0..=2 => MapAction::SelectMap(button.id.clone()),
                    3 => MapAction::Back,
                    _ => MapAction::None,
                };
            }
        }
        MapAction::None
    }
}
