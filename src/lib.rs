//lime project intern modules

pub mod settings;
pub mod windows;
pub mod editor;
pub mod program;
pub mod utils;

pub use settings::Settings;
pub use windows::Window;
pub use editor::Editor;
pub use program::Program;
pub use utils::*;

//lime global project utilities

use serde::{Deserialize, Serialize};
use std::{path::{Component, Path, PathBuf}, sync::atomic::{AtomicU64, Ordering}};
use macroquad::{color::Color, window::Conf};

pub const DEFAULT_WINDOW_WIDTH: i32 = 1200;
pub const DEFAULT_WINDOW_HEIGHT: i32 = 800;
pub const DIRECTORY_APP_NAME: &str = "block-programming";
pub const DISPLAY_APP_NAME: &str = "Block Programming";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UUID(u64);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl UUID {
    pub fn new() -> Self {
        UUID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        window_resizable: true,
        window_title: DISPLAY_APP_NAME.to_string(),
        ..Default::default()
    }
}

pub fn config_path(path_str: &str) -> PathBuf {
    build(dirs::config_dir().expect("no config directory found"), path_str)
}

pub fn data_path(path_str: &str) -> PathBuf {
    build(dirs::data_dir().expect("no data directory found"), path_str)
}

fn build(base: PathBuf, path_str: &str) -> PathBuf {
    let mut path = base.join(DIRECTORY_APP_NAME);
    for component in Path::new(path_str).components() {
        if let Component::Normal(part) = component {
            path.push(part);
        }
    }

    path
}

#[allow(
    non_camel_case_types
)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    en,
    de,
}

pub trait ColorExt {
    fn brightness(self, percent: f32) -> Self;
}

impl ColorExt for Color {
    fn brightness(self, percent: f32) -> Self {
        let p = (percent / 100.0).clamp(-1.0, 1.0);

        let adjust = |c: f32| {
            if p >= 0.0 {
                c + (1.0 - c) * p
            } else {
                c * (1.0 + p)
            }
        };

        Color::new(adjust(self.r), adjust(self.g), adjust(self.b), self.a)
    }
}