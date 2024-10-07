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
    // lf: Box<dyn ImageStream>,
    lf: Box<Arc<DynamicImageStream>>,
    // lb: Box<dyn ImageStream>,
    lb: Box<Arc<DynamicImageStream>>,
    // rf: Box<dyn ImageStream>,
    rf: Box<Arc<DynamicImageStream>>,
    // rb: Box<dyn ImageStream>,
    rb: Box<Arc<DynamicImageStream>>,
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
            // lf: StaticImageStream::new(include_bytes!("../local-images/segmentable/l/example-foreground.png")),
            lf: DynamicImageStream::new("http://172.16.16.173:81/stream"),
            lb: DynamicImageStream::new("http://172.16.16.192:81/stream"),
            // lb: StaticImageStream::new(include_bytes!("../local-images/segmentable/l/example-background.png")),
            // rf: StaticImageStream::new(include_bytes!("../local-images/segmentable/r/example-foreground.png")),
            rf: DynamicImageStream::new("http://172.16.16.163:81/stream"),
            rb: DynamicImageStream::new("http://172.16.16.191:81/stream"),
            // rb: StaticImageStream::new(include_bytes!("../local-images/segmentable/r/example-background.png")),
        }
    }

    fn system_meets_requirements(&self) -> bool {
        // check if cuda is available using ORT
        self.cache.is_gpu()
    }

    pub fn images(&mut self, re_segment: bool) -> ((DynamicImage, DynamicImage), (DynamicImage, DynamicImage)) {
        let mut overlay = if self.system_meets_requirements() {
            self.cache.segment(vec![&self.rf.image(), &self.lf.image()], re_segment)
        } else {
            vec![DynamicImage::new_rgba8(self.rf.image().width(), self.rf.image().height()); 2]
        };

        (
            (
                overlay.remove(0),
                self.lb.image()
            ),
            (
                overlay.remove(0),
                self.rb.image()
            )
        )
    }
}


pub fn dynamic_to_ggez(ctx: &mut Context, image: DynamicImage) -> Image {
    Image::from_pixels(ctx, image.clone().to_rgba8().as_bytes(), ImageFormat::Rgba8UnormSrgb, image.width(), image.height())
}