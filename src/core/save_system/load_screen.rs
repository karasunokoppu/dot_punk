use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use bevy::prelude::*;

use crate::core::setting::game_setting::SAVE_DIR;

//TODO [ContinueToPlayを選択した際にセーブデータの選択画面を作る]
// セーブデータの一覧を保持するリソース
#[derive(Resource, Default)]
pub struct SaveFileDatas{
    pub save_datas: Vec<SaveFileData>
}

#[derive(Component)]
pub struct SaveFileData {
    // pub save_name: String,
    pub dir: String,
}

// セーブファイルの一覧を取得してSaveFileDatasに格納する
pub fn load_save_files(
    mut r_save_file_datas: ResMut<SaveFileDatas>,
){
    if !Path::new(format!("./{}", SAVE_DIR).as_str()).is_dir(){
        eprintln!("Save directory not found");
        return;
    }else{
        for entry in fs::read_dir(Path::new(format!("./{}", SAVE_DIR).as_str())).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                    if ext.eq_ignore_ascii_case("ron") {
                        let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                        r_save_file_datas.save_datas.push(SaveFileData { dir: file_name });
                    }
                }
            }
        }
    }
}