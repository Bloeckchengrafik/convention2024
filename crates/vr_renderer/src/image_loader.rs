use std::io::Cursor;
use ggez::Context;
use ggez::graphics::{Image, ImageFormat};
use image::{DynamicImage, EncodableLayout};
use image::io::Reader as ImageReader;
use crate::segmentation::segment_merge;

fn load_image(bytes: &[u8]) -> DynamicImage {
    ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap()
}

pub fn left_image() -> DynamicImage {
    segment_merge(
        load_image(include_bytes!("../local-images/segmentable/l/example-foreground.png")),
        load_image(include_bytes!("../local-images/segmentable/l/example-background.png")),
    )
}

pub fn right_image() -> DynamicImage {
    segment_merge(
        load_image(include_bytes!("../local-images/segmentable/r/example-foreground.png")),
        load_image(include_bytes!("../local-images/segmentable/r/example-background.png")),
    )
}

pub fn dynamic_to_ggez(ctx: &mut Context, image: DynamicImage) -> Image {
    Image::from_pixels(ctx, image.clone().to_rgba8().as_bytes(), ImageFormat::Rgba8UnormSrgb, image.width(), image.height())
}