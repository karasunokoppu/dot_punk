
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use crate::core::{resource::{ActiveDatas, InWorldTime}, setting::game_setting::SAVE_DIR};

#[derive(Event)]
pub struct LoadDataEvent {
    pub save_name: String,
}

#[derive(Event)]
pub struct SaveDataEvent;

#[derive(Serialize, Deserialize)]
pub struct SavedDatas{
    active_datas: ActiveDatas,
    in_world_time: InWorldTime,
}

//TODO [InWorldTimeも保存するようにする]
//TODO [SaveSystemPluginを作成し実装する]
pub fn save_data(
    active_datas: &ActiveDatas,
    in_world_time: &InWorldTime,
    mut save_data_event: EventReader<SaveDataEvent>,
) {
    for _ in save_data_event.read(){
        let file_path = format!("{}/{}.ron", SAVE_DIR, save_data_naming_convention());

        let data = SavedDatas{
            active_datas: active_datas.clone(),
            in_world_time: in_world_time.clone()
        };

        let save_strings = ron::to_string(&data).expect("Failed to serialize data");
        // Create the save directory if it doesn't exist
        fs::create_dir_all(SAVE_DIR).unwrap();
        // Write the serialized data to a file
        let mut file = File::create(file_path).unwrap();

        file.write_all(save_strings.as_bytes()).unwrap();
    }
}

//TODO [InWorldTimeも読み込むようにする]
//TODO [SaveSystemPluginを作成し実装する]
pub fn load_data(
    mut active_datas: ResMut<ActiveDatas>,
    mut in_world_time: ResMut<InWorldTime>,
    mut load_data_event: EventReader<LoadDataEvent>,
) {
    for load_data_index in load_data_event.read() {
        let file_path = format!("saves/{}.ron", load_data_index.save_name);
        let mut file = File::open(file_path).expect("Failed to open save file");
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();
        let load_data:SavedDatas =
            ron::from_str(&buffer).expect("Failed to deserialize data");
        active_datas.active_stage_id = load_data.active_datas.active_stage_id;
        active_datas.active_stage_name = load_data.active_datas.active_stage_name;
        active_datas.teleport_stage = load_data.active_datas.teleport_stage;
        active_datas.teleport_position = load_data.active_datas.teleport_position;
        active_datas.closest_interact_entity_type = load_data.active_datas.closest_interact_entity_type;
        active_datas.talking_npc = load_data.active_datas.talking_npc;
        active_datas.talk_index = load_data.active_datas.talk_index;

        in_world_time.hour = load_data.in_world_time.hour;
        in_world_time.minute = load_data.in_world_time.minute;
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