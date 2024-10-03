use image::DynamicImage;
use messages::EyeSettings;

pub fn postprocess(image: DynamicImage, eye_settings: &EyeSettings) -> DynamicImage {
    image
        .resize_to_fill(
            eye_settings.image_width,
            eye_settings.image_height,
            image::imageops::FilterType::Nearest
        )
}