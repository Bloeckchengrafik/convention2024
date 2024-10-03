use image::{DynamicImage, GrayImage};
use messages::ModelConfiguration;
use crate::models::SegmentationModel;
use crate::models::yolo::{Args, YOLOResult, YOLO};
use crate::profiling::Profiler;

pub enum YoloV8ONNXQuantization {
    Int8,
    Half,
    None,
}

pub enum YoloV8ONNXModelType {
    V8m(YoloV8ONNXQuantization),
    V11s(YoloV8ONNXQuantization),
    V11m(YoloV8ONNXQuantization),
}

impl YoloV8ONNXQuantization {
    fn to_model_pathspec(&self) -> String {
        match self {
            YoloV8ONNXQuantization::Int8 => {
                "-int8".to_string()
            }
            YoloV8ONNXQuantization::Half => {
                "-half".to_string()
            }
            YoloV8ONNXQuantization::None => {
                "".to_string()
            }
        }
    }
}


impl YoloV8ONNXModelType {
    fn to_model_path(&self) -> String {
        match self {
            YoloV8ONNXModelType::V8m(q) => { format!("./onnx/yolov8m-seg{}.onnx", q.to_model_pathspec()).to_string() }
            YoloV8ONNXModelType::V11s(q) => { format!("./onnx/yolo11s-seg{}.onnx", q.to_model_pathspec()).to_string() }
            YoloV8ONNXModelType::V11m(q) => { format!("./onnx/yolo11m-seg{}.onnx", q.to_model_pathspec()).to_string() }
        }
    }
}
pub struct YoloONNXSegmentationModel {
    model: YOLO,
}

impl YoloONNXSegmentationModel {
    pub fn new(mtype: YoloV8ONNXModelType, model_configuration: &ModelConfiguration) -> Self {
        let yolo = YOLO::new(Args::new(mtype.to_model_path(), &model_configuration)).unwrap();
        yolo.summary();

        Self {
            model: yolo
        }
    }
}

impl SegmentationModel for YoloONNXSegmentationModel {
    fn predict(&mut self, images: &Vec<DynamicImage>) -> Vec<GrayImage> {
        let mut profiler = Profiler::new(false);
        let images = images.iter()
            .map(|it|
                it.resize(
                    640, 640,
                    image::imageops::FilterType::Nearest,
                )
            ).collect::<Vec<DynamicImage>>();
        profiler.print_elapsed("resize");

        let allowed_ids = self.model
            .names()
            .iter()
            .enumerate()
            .filter(|(_, it)| it.clone() == "person" || it.clone() == "clock")
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        profiler.print_elapsed("filter");

        let ys_result = self.model.run(&images).unwrap();

        profiler.print_elapsed("run");

        let result = ys_result
            .iter()
            .zip(images.iter())
            .map(|(result, img)| {
                let mut profiler = Profiler::new(false);
                if result.bboxes.is_none() || result.masks.is_none() {
                    return GrayImage::new(img.width(), img.height());
                }

                profiler.print_elapsed("bbox_mask_none");

                let bboxes = result.bboxes.clone().unwrap();
                let masks = result.masks.clone().unwrap();
                profiler.print_elapsed("clone");
                let mut final_mask = GrayImage::new(img.width(), img.height());
                let width = img.width() as usize;

                profiler.print_elapsed("new");

                for (mask, bbox) in masks.iter().zip(bboxes) {
                    if !allowed_ids.contains(&bbox.id()) {
                        continue
                    }

                    for (id, px) in mask.iter().enumerate() {
                        let x = id % width;
                        let y = id / width;

                        if *px != 0 {
                            final_mask.put_pixel(x as u32, y as u32, image::Luma([255]));
                        }
                    }
                }

                profiler.print_elapsed("mask");
                final_mask
            }).collect();

        profiler.print_elapsed("postprocess");

        result
    }
}