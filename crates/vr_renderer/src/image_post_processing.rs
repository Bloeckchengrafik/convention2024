use image::DynamicImage;
use crate::render_settings::EyeSettings;

pub fn postprocess(image: DynamicImage, eye_settings: &EyeSettings) -> DynamicImage {
    image
        .resize_to_fill(eye_settings.image_width, eye_settings.image_height, image::imageops::FilterType::CatmullRom)
        .adjust_contrast(0.9)
}