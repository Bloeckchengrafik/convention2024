use crate::models::yolonnx::YoloV8ONNXModelType::*;
use crate::models::yolonnx::YoloV8ONNXQuantization::*;
use crate::models::yolonnx::YoloONNXSegmentationModel;
use image::{DynamicImage, GrayImage};
use messages::{ModelType, RenderSettingsData};

pub mod yolo;
mod yolonnx;


pub trait SegmentationModel {
    fn predict(&mut self, images: &Vec<DynamicImage>) -> Vec<Option<GrayImage>>;
    fn is_gpu(&self) -> bool;
}

pub fn load_model(config: &RenderSettingsData) -> Box<dyn SegmentationModel> {
    Box::new(
        match config.model {
            ModelType::YoloV8mInt8ONNX => { YoloONNXSegmentationModel::new(V8m(Int8), &config.model_configuration) }
            ModelType::YoloV8mHalfONNX => { YoloONNXSegmentationModel::new(V8m(Half), &config.model_configuration) }
            ModelType::YoloV8mFullONNX => { YoloONNXSegmentationModel::new(V8m(None), &config.model_configuration) }
            ModelType::YoloV11sInt8ONNX => { YoloONNXSegmentationModel::new(V11s(Int8), &config.model_configuration) }
            ModelType::YoloV11sHalfONNX => { YoloONNXSegmentationModel::new(V11s(Half), &config.model_configuration) }
            ModelType::YoloV11sFullONNX => { YoloONNXSegmentationModel::new(V11s(None), &config.model_configuration) }
            ModelType::YoloV11mInt8ONNX => { YoloONNXSegmentationModel::new(V11m(Int8), &config.model_configuration) }
            ModelType::YoloV11mHalfONNX => { YoloONNXSegmentationModel::new(V11m(Half), &config.model_configuration) }
            ModelType::YoloV11mFullONNX => { YoloONNXSegmentationModel::new(V11m(None), &config.model_configuration) }
        }
    )
}
