use image::{DynamicImage, GenericImageView};
use crate::yolov8::{Args, YOLOv8};

pub fn segment_merge(hand: DynamicImage, base: DynamicImage) -> DynamicImage {
    let mut yolo = YOLOv8::new(Args::new("./onnx/yolov8m-seg.onnx".to_string())).unwrap();
    yolo.summary();
    let xs = vec![hand];
    let ys = yolo.run(&xs);
    println!("{:?}", ys);
    return base;
}

fn extract() {}