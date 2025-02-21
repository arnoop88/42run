use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::WorldState;
use crate::maps::Maps;
use crate::skins::Skins;

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub unlocked_maps: HashMap<String, bool>,
    pub unlocked_skins: HashMap<String, bool>,
    pub quest_progress: HashMap<String, i32>,
 	pub current_skin: Skins,
 	pub current_map: Maps,
}

pub fn save_progress(save_data: &SaveData) -> io::Result<()> {
	let json = serde_json::to_string_pretty(save_data)
		.expect("Failed to serialize save data");
	let mut file = fs::File::create("target/game_data.json")?;
	file.write_all(json.as_bytes())?;
	Ok(())
}

pub fn load_progress() -> io::Result<SaveData> {
	let data = fs::read_to_string("target/game_data.json")?;
	let save_data = serde_json::from_str(&data)
		.expect("Failed to deserialize save data");
	Ok(save_data)
}

pub fn extract_save_data(world: &WorldState) -> SaveData {
	SaveData {
		unlocked_maps: world.unlocked_maps.clone(),
		unlocked_skins: world.unlocked_skins.clone(),
		quest_progress: world.quest_progress.clone(),
		current_skin: world.current_skin.clone(),
        current_map: world.current_map.clone(),
	}
}