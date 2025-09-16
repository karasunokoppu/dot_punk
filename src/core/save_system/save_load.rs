
use bevy::prelude::*;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use crate::core::resource::ActiveDatas;

#[derive(Event)]
pub struct LoadDataEvent {
    pub save_name: String,
}
//TODO [InWorldTimeも保存するようにする]
pub fn save_data(data: &ActiveDatas) {
    let file_path = format!("saves/{}.ron", save_data_naming_convention());
    let save_strings = ron::to_string(data).expect("Failed to serialize data");
    let mut file = File::create(file_path).unwrap();

    file.write_all(save_strings.as_bytes()).unwrap();
}

//TODO [InWorldTimeも読み込むようにする]
pub fn load_data(
    mut data: ResMut<ActiveDatas>,
    mut save_data_iter: EventReader<LoadDataEvent>,
) {
    for load_data_index in save_data_iter.read() {
        let file_path = format!("saves/{}.ron", load_data_index.save_name);
        let mut file = File::open(file_path).unwrap();
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();
        let load_data: ActiveDatas =
            ron::from_str(&buffer).expect("Failed to deserialize data");

        data.active_stage_id = load_data.active_stage_id;
        data.active_stage_name = load_data.active_stage_name;
        data.teleport_stage = load_data.teleport_stage;
        data.teleport_position = load_data.teleport_position;
        data.closest_interact_entity_type = load_data.closest_interact_entity_type;
        data.talking_npc = load_data.talking_npc;
        data.talk_index = load_data.talk_index;
    }
}

pub fn count_ron_files_in_save_dir() -> std::io::Result<usize> {
    let save_dir = Path::new("saves");

    if !save_dir.exists() || !save_dir.is_dir() {
        return Ok(0); // ディレクトリが存在しない場合は0を返す
    }

    let count = fs::read_dir(save_dir)?
        .filter_map(Result::ok) // 読み込みに失敗したファイルをスキップ
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| ext == "ron") // .ron拡張子を持つか確認
        })
        .count();

    Ok(count)
}

pub fn save_data_naming_convention() -> String{
    let file_count: u32 = match count_ron_files_in_save_dir() {
        Ok(count) => count as u32,
        Err(e) => panic!("{e}"),
    };
    (file_count + 1).to_string()
}