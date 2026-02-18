use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

const HIGHSCORE_FILENAME: &str = "highscore.json";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HighScore {
    pub best_score: u32,
}

impl HighScore {
    fn get_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "axelfrache", "ratatui-snake") {
             let data_dir = proj_dirs.data_dir();
             if !data_dir.exists() {
                 let _ = fs::create_dir_all(data_dir);
             }
             return Some(data_dir.join(HIGHSCORE_FILENAME));
        }
        None
    }

    pub fn load() -> Self {
        if let Some(path) = Self::get_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(record) = serde_json::from_str(&content) {
                        return record;
                    }
                }
            }
        }
        HighScore::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::get_path() {
            if let Ok(content) = serde_json::to_string(self) {
                let _ = fs::write(path, content);
            }
        }
    }
}
