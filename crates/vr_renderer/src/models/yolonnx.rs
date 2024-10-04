use image::{DynamicImage, GrayImage};
use tracing::trace;
use messages::ModelConfiguration;
use crate::models::SegmentationModel;
use crate::models::yolo::{Args, YOLOResult, YOLO};

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
        let _span = tracing::trace_span!("YoloONNXSegmentationModel::predict");
        let images = images.iter()
            .map(|it|
                it.resize(
                    640, 640,
                    image::imageops::FilterType::Nearest,
                )
            ).collect::<Vec<DynamicImage>>();
        trace!("Resized images");

        let ys_result = self.model.run(&images).unwrap();

        trace!("Got result");

        let result = ys_result
            .iter()
            .zip(images.iter())
            .map(|(result, img)| {
                let _span = tracing::trace_span!("YoloONNXSegmentationModel::postprocess");
                if result.bboxes.is_none() || result.masks.is_none() {
                    return GrayImage::new(img.width(), img.height());
                }

                trace!("Got bboxes and masks");

                let masks = result.masks.clone().unwrap();
                trace!("Cloned masks");

                let mut final_mask = GrayImage::new(img.width(), img.height());
                let width = img.width() as usize;

                trace!("Created final mask");

                for mask in masks.iter() {
                    for (id, px) in mask.iter().enumerate() {
                        let x = id % width;
                        let y = id / width;

                        if *px != 0 {
                            final_mask.put_pixel(x as u32, y as u32, image::Luma([255]));
                        }
                    }
                }

                trace!("Processed masks");
                final_mask
            }).collect();

        trace!("Returning result");

        result
    }
}