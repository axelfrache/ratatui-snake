use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const HIGHSCORE_FILE: &str = "highscore.json";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HighScore {
    pub best_score: u32,
}

impl HighScore {
    pub fn load() -> Self {
        if Path::new(HIGHSCORE_FILE).exists() {
            if let Ok(content) = fs::read_to_string(HIGHSCORE_FILE) {
                if let Ok(record) = serde_json::from_str(&content) {
                    return record;
                }
            }
        }
        HighScore::default()
    }

    pub fn save(&self) {
        if let Ok(content) = serde_json::to_string(self) {
            let _ = fs::write(HIGHSCORE_FILE, content);
        }
    }
}
