use std::fs::File;
use serde::Deserialize;

const UNIT_DATA_PATH: &'static str = "./resources/UnitData.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    pub name: String,
    pub health_bar_height: f32,
    pub base_move_speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub attack_speed_ratio: f32,
    pub acquisition_range: f32,
    pub selection_radius: f32,
    pub pathing_radius: f32,
    pub gameplay_radius: f32,
    pub basic_atk_missile_speed: f32,
    pub basic_atk_windup: f32
}

pub struct UnitData {
    pub champions: Vec<Champion>,
}

impl UnitData {
    pub fn new() -> UnitData {
        let mut tmp = UnitData { champions: vec![] };
        let file = File::open(UNIT_DATA_PATH);
        tmp.champions = serde_json::from_reader(file.unwrap()).expect("Failed to parse.");
        return tmp;
    }

    pub fn get_champion(&mut self, name: &str) -> &Champion {
        let champion: Vec<&Champion> = self.champions.iter().filter(|voc| voc.name == name).collect();
        return champion[0];
    }
}