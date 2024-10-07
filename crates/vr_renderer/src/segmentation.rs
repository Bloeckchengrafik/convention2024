use std::fmt::{Debug, Formatter};
use crate::models::{load_model, SegmentationModel};
use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Rgba, RgbaImage};
use messages::file_config::read_config;
use itertools::izip;
use tracing::{debug_span, instrument, trace, trace_span};
use messages::RenderSettingsData;

pub struct SegmentationCache {
    model: Box<dyn SegmentationModel>,
    last_masks: Vec<GrayImage>,
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
            model: load_model(&config),
            last_masks: vec![],
        }
    }
    pub fn reload(&mut self, config: &RenderSettingsData) {
        self.model = load_model(&config)
    }

    fn apply_mask(image: &DynamicImage, mask: &GrayImage) -> DynamicImage {
        let _span = debug_span!("apply_mask").entered();
        let mut overlay = image.clone().to_rgba8();
        overlay.pixels_mut().zip(mask.pixels()).for_each(|(pixel, mask_pixel)| {
            let mask_pixel = mask_pixel[0];
            pixel.0[3] = if mask_pixel > 150 {
                255
            } else {
                mask_pixel
            };
        });
        overlay.into()
    }

    fn mask_postprocess(mask: GrayImage) -> GrayImage {
        let _span = debug_span!("mask_postprocess").entered();
        // Blur edges of mask to avoid sharp transitions
        imageproc::filter::gaussian_blur_f32(&mask, 2.0)
    }

    pub fn segment(&mut self, hand: Vec<&DynamicImage>, re_segment: bool) -> Vec<DynamicImage> {
        let _span = debug_span!("segment_merge").entered();
        let sized_hand = debug_span!("sized_hand").in_scope(|| hand
            .iter()
            .map(
                |it| it.resize(
                    640, 640,
                    image::imageops::FilterType::Nearest,
                )
            ));

        let hands = debug_span!("collect_hands").in_scope(|| sized_hand.collect());
        let masks = debug_span!("predict").in_scope(|| {
            if re_segment || self.last_masks.len() != hand.len() {
                self.model.predict(&hands)
                    .iter()
                    .enumerate()
                    .map(|(id, optional_mask)| {
                        if let Some(mask) = optional_mask {
                            let mask = mask.clone();
                            if self.last_masks.len() > id {
                                self.last_masks[id] = mask.clone();
                            } else {
                                self.last_masks.push(mask.clone());
                            }
                            mask
                        } else {
                            if self.last_masks.len() > id {
                                self.last_masks[id].clone()
                            } else {
                                GrayImage::new(1, 1)
                            }
                        }
                    }).collect()
            } else {
                self.last_masks.clone()
            }
        });
        debug_span!("apply_masks").in_scope(|| masks
            .into_iter()
            .zip(hands.iter())
            .map(
                |(mask, hand)|
                    Self::apply_mask(
                        &hand,
                        &Self::mask_postprocess(mask),
                    )
            ).collect()
        )
    }

    pub fn is_gpu(&self) -> bool {
        self.model.is_gpu()
    }
}
