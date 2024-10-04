use std::fmt::{Debug, Formatter};
use crate::models::{load_model, SegmentationModel};
use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Rgba, RgbaImage};
use messages::file_config::read_config;
use itertools::izip;
use tracing::{instrument, trace, trace_span};
use messages::RenderSettingsData;

pub struct SegmentationCache {
    model: Box<dyn SegmentationModel>,
}

impl Debug for SegmentationCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegmentationCache").finish()
    }
}

impl SegmentationCache {
    pub fn new() -> Self {
        let config = read_config();

        Self {
            model: load_model(&config)
        }
    }
    pub fn reload(&mut self, config: &RenderSettingsData) {
        self.model = load_model(&config)
    }

    fn overlay_images(background: &DynamicImage, foreground: &DynamicImage, mask: &GrayImage) -> DynamicImage {
        let (width, height) = background.dimensions();
        let (fg_w, fg_h) = foreground.dimensions();
        if width != fg_w || height != fg_h {
            panic!("Images must have the same dimensions: background: {}x{}, foreground: {}x{}", width, height, fg_w, fg_h);
        }
        let mut output: RgbaImage = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let bg_pixel = background.get_pixel(x, y).0; // Get background pixel
                let fg_pixel = foreground.get_pixel(x, y).0; // Get foreground pixel
                let mask_pixel = mask.get_pixel(x, y).0; // Get mask pixel

                // Use the mask to determine the output pixel
                let alpha = mask_pixel[0] as f32 / 255.0; // Normalize mask value to [0, 1]
                let blended_pixel = [
                    (bg_pixel[0] as f32 * (1.0 - alpha) + fg_pixel[0] as f32 * alpha) as u8,
                    (bg_pixel[1] as f32 * (1.0 - alpha) + fg_pixel[1] as f32 * alpha) as u8,
                    (bg_pixel[2] as f32 * (1.0 - alpha) + fg_pixel[2] as f32 * alpha) as u8,
                    255, // Set alpha to opaque
                ];

                output.put_pixel(x, y, Rgba(blended_pixel));
            }
        }

        DynamicImage::ImageRgba8(output)
    }

    fn mask_postprocess(mask: GrayImage) -> GrayImage {
        let _span = trace_span!("mask_postprocess").entered();
        // Blur edges of mask to avoid sharp transitions
        imageproc::filter::gaussian_blur_f32(&mask, 2.0)
    }

    pub fn segment_merge(&mut self, hand: Vec<&DynamicImage>, base: Vec<&DynamicImage>) -> Vec<DynamicImage> {
        let _span = trace_span!("segment_merge").entered();
        let sized_hand = trace_span!("sized_hand").in_scope(|| hand
            .iter()
            .map(
                |it| it.resize(
                    640, 640,
                    image::imageops::FilterType::Nearest,
                )
            ));

        let sized_bg = trace_span!("sized_bg").in_scope(|| base
            .iter()
            .map(
                |it| it.resize(
                    640, 640,
                    image::imageops::FilterType::Nearest,
                )
            ));

        let hands = trace_span!("collect_hands").in_scope(|| sized_hand.collect());

        let masks = trace_span!("predict").in_scope(|| self.model.predict(&hands));

        let images = izip!(sized_bg, hands, masks)
            .map(
                |(base, hand, mask)| {
                    let _span = trace_span!("overlay").entered();
                    SegmentationCache::overlay_images(&base, &hand, &Self::mask_postprocess(mask))
                    // hand
                }
            ).collect();

        images
    }
}
