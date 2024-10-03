use ron::ser::PrettyConfig;
use crate::{RenderSettingsData};

fn write_default_config() {
    let conf: RenderSettingsData = Default::default();
    let string = ron::ser::to_string_pretty(&conf, Default::default()).unwrap();
    std::fs::write("conf.ron", string).unwrap();
}

pub fn read_config() -> RenderSettingsData {
    match std::fs::read_to_string("conf.ron") {
        Ok(string) => {
            ron::de::from_str(&string).unwrap()
        }
        Err(_) => {
            write_default_config();
            Default::default()
        }
    }
}

pub fn save_config(settings: &RenderSettingsData) {
    let string = ron::ser::to_string_pretty(&settings, PrettyConfig::default()).unwrap();
    std::fs::write("conf.ron", string).unwrap();
}
