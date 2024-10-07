use ggez::glam::Vec2;
use messages::{EyeSettings, RenderSettingsData};

pub struct TransformSet {
    pub position_left_front: Vec2,
    pub position_left_back: Vec2,
    pub position_right_front: Vec2,
    pub position_right_back: Vec2,
}

const HEIGHT: i32 = 480;
const WIDTH: i32 = 800;

fn vertical_center(eye_settings: &EyeSettings) -> f32 {
    let combined_margin = HEIGHT - eye_settings.image_height as i32;
    (combined_margin / 2) as f32
}

impl TransformSet {
    pub fn from(config: &RenderSettingsData) -> Self {
        TransformSet {
            position_left_back: Vec2 {
                x: (WIDTH / 2 - config.space_between_back - (config.left_eye.image_width as i32)) as f32,
                y: vertical_center(&config.left_eye) + config.v_offset as f32,
            },
            position_right_back: Vec2 {
                x: (WIDTH / 2 + config.space_between_back) as f32,
                y: vertical_center(&config.right_eye) + config.v_offset as f32,
            },

            position_left_front: Vec2 {
                x: (WIDTH / 2 - config.space_between_front - (config.left_eye.image_width as i32)) as f32,
                y: vertical_center(&config.left_eye) + config.v_offset as f32,
            },
            position_right_front: Vec2 {
                x: (WIDTH / 2 + config.space_between_front) as f32,
                y: vertical_center(&config.right_eye) + config.v_offset as f32,
            },
        }
    }
}