use ggez::glam::Vec2;
use crate::render_settings::{EyeSettings, RenderSettingsData};

pub struct TransformSet {
    pub position_left: Vec2,
    pub position_right: Vec2,
}

const HEIGHT: i32 = 480;
const WIDTH: i32 = 800;

fn vertical_center(eye_settings: &EyeSettings) -> f32 {
    let combined_margin = HEIGHT - eye_settings.image_height as i32;
    return (combined_margin / 2) as f32;
}

impl TransformSet {
    pub fn from(config: &RenderSettingsData) -> Self {
        TransformSet {
            position_left: Vec2 {
                x: (WIDTH / 2 - config.space_between - (config.left_eye.image_width as i32)) as f32,
                y: vertical_center(&config.left_eye) + config.v_offset as f32,
            },
            position_right: Vec2 {
                x: (WIDTH / 2 + config.space_between) as f32,
                y: vertical_center(&config.right_eye) + config.v_offset as f32,
            },
        }
    }
}