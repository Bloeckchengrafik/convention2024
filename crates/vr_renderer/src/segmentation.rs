use crate::models::{load_model, SegmentationModel};
use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Rgba, RgbaImage};
use messages::file_config::read_config;
use itertools::izip;
use messages::RenderSettingsData;
use crate::profiling::Profiler;

pub struct SegmentationCache {
    model: Box<dyn SegmentationModel>,
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
        // Blur edges of mask to avoid sharp transitions
        imageproc::filter::gaussian_blur_f32(&mask, 2.0)
    }

    pub fn segment_merge(&mut self, hand: Vec<DynamicImage>, base: Vec<DynamicImage>) -> Vec<DynamicImage> {
        let mut profiler = Profiler::new(false);
        let sized_hand = hand
            .iter()
            .map(
                |it| it.resize(
                    640, 640,
                    image::imageops::FilterType::Nearest,
                )
            );
        profiler.print_elapsed("sz_hand");
        let sized_bg = base
            .iter()
            .map(
                |it| it.resize(
                    640, 640,
                    image::imageops::FilterType::Nearest,
                )
            );
        profiler.print_elapsed("sz_bg");
        let hands = sized_hand.collect();
        profiler.print_elapsed("collect");

        let masks = self.model.predict(&hands);

        profiler.print_elapsed("predict");

        let images = izip!(sized_bg, hands, masks)
            .map(
                |(base, hand, mask)| {
                    SegmentationCache::overlay_images(&base, &hand, &Self::mask_postprocess(mask))
                    // return DynamicImage::from(mask)
                }
            ).collect();

        profiler.print_elapsed("overlay");

        images
    }
}
