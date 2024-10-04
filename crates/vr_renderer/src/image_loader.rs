use crate::imgstream::{DynamicImageStream, ImageStream, StaticImageStream};
use crate::segmentation::SegmentationCache;
use ggez::graphics::{Image, ImageFormat};
use ggez::Context;
use image::{DynamicImage, EncodableLayout};
use messages::RenderSettingsData;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct ImageLoader {
    cache: SegmentationCache,
    lf: Box<dyn ImageStream>,
    // lf: Box<Arc<DynamicImageStream>>,
    lb: Box<dyn ImageStream>,
    rf: Box<dyn ImageStream>,
    // rf: Box<Arc<DynamicImageStream>>,
    rb: Box<dyn ImageStream>,
}

impl ImageLoader {
    pub(crate) fn reload(&mut self, settings: &RenderSettingsData) {
        self.cache.reload(settings);
    }
}

impl Debug for ImageLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageLoader").finish()
    }
}

impl ImageLoader {
    pub fn new() -> Self {
        Self {
            cache: SegmentationCache::new(),
            lf: StaticImageStream::new(include_bytes!("../local-images/segmentable/l/example-foreground.png")),
            // lf: DynamicImageStream::new("http://172.16.16.173:81/stream"),
            lb: StaticImageStream::new(include_bytes!("../local-images/segmentable/l/example-background.png")),
            rf: StaticImageStream::new(include_bytes!("../local-images/segmentable/r/example-foreground.png")),
            // rf: DynamicImageStream::new("http://172.16.16.163:81/stream"),
            rb: StaticImageStream::new(include_bytes!("../local-images/segmentable/r/example-background.png")),
        }
    }

    pub fn images(&mut self) -> (DynamicImage, DynamicImage) {
        let mut imgs = self.cache.segment_merge(
            vec![&self.rf.image(), &self.lf.image()],
            vec![&self.rb.image(), &self.lb.image()],
        );
        (imgs.pop().unwrap().clone(), imgs.pop().unwrap())
    }
}


pub fn dynamic_to_ggez(ctx: &mut Context, image: DynamicImage) -> Image {
    Image::from_pixels(ctx, image.clone().to_rgba8().as_bytes(), ImageFormat::Rgba8UnormSrgb, image.width(), image.height())
}