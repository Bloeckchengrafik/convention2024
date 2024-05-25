use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EyeSettings {
    pub image_width: u32,
    pub image_height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderSettingsData {
    pub left_eye: EyeSettings,
    pub right_eye: EyeSettings,
    pub v_offset: i32,
    pub space_between: i32,
}

pub struct SettingsFrame {
    pub data: RenderSettingsData,
    pub file_last_modified: std::time::SystemTime,
}

fn default_settings() -> RenderSettingsData {
    RenderSettingsData {
        left_eye: EyeSettings {
            image_width: 400,
            image_height: 480,
        },
        right_eye: EyeSettings {
            image_width: 400,
            image_height: 480,
        },
        v_offset: 0,
        space_between: 0,
    }
}

fn write_default_config() {
    let string = ron::ser::to_string_pretty(&default_settings(), Default::default()).unwrap();
    std::fs::write("config.ron", string).unwrap();
}

pub fn read_config() -> SettingsFrame {
    match std::fs::read_to_string("config.ron") {
        Ok(string) => {
            SettingsFrame {
                data: ron::de::from_str(&string).unwrap(),
                file_last_modified: std::fs::metadata("config.ron").unwrap().modified().unwrap(),
            }
        }
        Err(_) => {
            write_default_config();
            SettingsFrame {
                data: default_settings(),
                file_last_modified: std::fs::metadata("config.ron").unwrap().modified().unwrap(),
            }
        }
    }
}

pub fn config_has_changed(settings: &SettingsFrame) -> bool {
    let modified = std::fs::metadata("config.ron").unwrap().modified().unwrap();
    modified != settings.file_last_modified
}
