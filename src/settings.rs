use std::{fs::File, io::{BufReader, BufWriter, Write}, path::Path};

use serde::{Deserialize, Serialize};
use macroquad::color::{Color, WHITE};
use crate::{Language, config_path};

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "Color")]
struct ColorDef {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(with = "ColorDef")]
    pub background_color: Color,
    pub lang: Language
}

const FILE_PATH: &str = "settings.json";

impl Settings {
    pub fn new() -> Self {
        Self {
            background_color: WHITE,
            lang: Language::en
        }
    }

    pub fn save(&self) {
        Settings::create_parent_dir();
        let path = config_path(FILE_PATH);
        let tmp = path.with_extension("json.tmp");
        let file = File::create(&tmp).expect("failed to create temp settings file");
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, self).expect("failed to write settings");
        writer.flush().expect("failed to flush settings");
        writer.get_ref().sync_all().expect("failed to sync settings");
        std::fs::rename(&tmp, &path).expect("failed to replace settings file");
    }

    pub fn load() -> Self {
        let path = config_path(FILE_PATH);
        let bak = path.with_extension("json.bak");

        if let Some(settings) = Settings::try_read(&path) {
            let _ = std::fs::copy(&path, &bak);
            return settings;
        }

        if let Some(settings) = Settings::try_read(&bak) {
            eprintln!("Settings file unusable, restored from backup");
            let _ = std::fs::rename(&path, path.with_extension("json.broken"));
            settings.save();
            return settings;
        }

        eprintln!("No valid settings found, creating defaults");
        let settings = Settings::new();
        settings.save();
        settings
    }

    fn try_read(path: &Path) -> Option<Settings> {
        let file = File::open(path).ok()?;
        serde_json::from_reader(BufReader::new(file)).ok()
    }

    fn create_parent_dir() {
        if let Some(parent) = config_path(FILE_PATH).parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| panic!("failed to create settings directory: {e}"));
        }
    }
}