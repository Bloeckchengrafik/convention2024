use std::io::Cursor;
use ggez::Context;
use ggez::graphics::{Image, ImageFormat};
use image::{DynamicImage, EncodableLayout};
use image::io::Reader as ImageReader;
use messages::RenderSettingsData;
use crate::segmentation::SegmentationCache;

fn load_image(bytes: &[u8]) -> DynamicImage {
    ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap()
}

pub struct ImageLoader {
    cache: SegmentationCache,
    lf: DynamicImage,
    lb: DynamicImage,
    rf: DynamicImage,
    rb: DynamicImage,
}

impl ImageLoader {
    pub(crate) fn reload(&mut self, settings: &RenderSettingsData) {
        self.cache.reload(settings);
    }
}

impl ImageLoader {
    pub fn new() -> Self {
        Self {
            cache: SegmentationCache::new(),
            lf: load_image(include_bytes!("../local-images/segmentable/l/example-foreground.png")),
            lb: load_image(include_bytes!("../local-images/segmentable/l/example-background.png")),
            rf: load_image(include_bytes!("../local-images/segmentable/r/example-foreground.png")),
            rb: load_image(include_bytes!("../local-images/segmentable/r/example-background.png")),
        }
    }

    pub fn images(&mut self) -> (DynamicImage, DynamicImage) {
        let mut imgs = self.cache.segment_merge(
            vec![self.lf.clone(), self.rf.clone()],
            vec![self.lb.clone(), self.rb.clone()],
        );
        (imgs.pop().unwrap(), imgs.pop().unwrap())
    }
}


pub fn dynamic_to_ggez(ctx: &mut Context, image: DynamicImage) -> Image {
    Image::from_pixels(ctx, image.clone().to_rgba8().as_bytes(), ImageFormat::Rgba8UnormSrgb, image.width(), image.height())
}