pub mod model;
mod yolo_result;
mod ort_backend;

use std::io::{Read, Write};
use ndarray::{Array, Ix1};
use messages::ModelConfiguration;
pub use crate::models::yolo::model::YOLO;
pub use crate::models::yolo::ort_backend::{Batch, OrtBackend, OrtConfig, OrtEP, YOLOTask};
pub use crate::models::yolo::yolo_result::{Bbox, Embedding, Point2, YOLOResult};

pub struct Args {
    pub model: String,
    pub device_id: u32,
    pub trt: bool,
    pub cuda: bool,
    pub batch: u32,
    pub batch_min: u32,
    pub batch_max: u32,
    pub fp16: bool,
    pub task: Option<YOLOTask>,
    pub nc: Option<u32>,
    pub nk: Option<u32>,
    pub nm: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub conf: f32,
    pub iou: f32,
    pub kconf: f32,
    pub plot: bool,
    pub profile: bool,
}

impl Args {
    // cargo run --release --  --trt --fp16 --model ../assets/weights/yolov8m-seg.onnx --source ../assets/images/0172.jpg --plot
    pub fn new(model_path: String, model_configuration: &ModelConfiguration) -> Self {
        Args {
            model: model_path,
            device_id: 0,
            trt: false,
            cuda: true,
            batch: 2,
            batch_min: 1,
            batch_max: 32,
            fp16: true,
            task: None,
            nc: None,
            nk: None,
            nm: None,
            width: Some(640),
            height: Some(640),
            conf: model_configuration.confidence,
            iou: model_configuration.iou,
            kconf: model_configuration.kconf,
            plot: false,
            profile: false,
        }
    }
}

pub fn non_max_suppression(
    xs: &mut Vec<(Bbox, Option<Vec<Point2>>, Option<Vec<f32>>)>,
    iou_threshold: f32,
) {
    xs.sort_by(|b1, b2| b2.0.confidence().partial_cmp(&b1.0.confidence()).unwrap());

    let mut current_index = 0;
    for index in 0..xs.len() {
        let mut drop = false;
        for prev_index in 0..current_index {
            let iou = xs[prev_index].0.iou(&xs[index].0);
            if iou > iou_threshold {
                drop = true;
                break;
            }
        }
        if !drop {
            xs.swap(current_index, index);
            current_index += 1;
        }
    }
    xs.truncate(current_index);
}

pub fn fastnms(
    xs: &mut Vec<(Bbox, Option<Array<f32, Ix1>>)>,
    iou_threshold: f32,
) {
    xs.sort_by(|b1, b2| {
        b2.0
            .confidence()
            .partial_cmp(&b1.0.confidence())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut current_index = 0;
    for index in 0..xs.len() {
        let mut drop = false;
        for prev_index in 0..current_index {
            let iou = xs[prev_index].0.iou(&xs[index].0);
            if iou > iou_threshold {
                drop = true;
                break;
            }
        }
        if !drop {
            xs.swap(current_index, index);
            current_index += 1;
        }
    }
    xs.truncate(current_index);
}


pub fn gen_time_string(delimiter: &str) -> String {
    let offset = chrono::FixedOffset::east_opt(8 * 60 * 60).unwrap(); // Beijing
    let t_now = chrono::Utc::now().with_timezone(&offset);
    let fmt = format!(
        "%Y{}%m{}%d{}%H{}%M{}%S{}%f",
        delimiter, delimiter, delimiter, delimiter, delimiter, delimiter
    );
    t_now.format(&fmt).to_string()
}

pub const SKELETON: [(usize, usize); 16] = [
    (0, 1),
    (0, 2),
    (1, 3),
    (2, 4),
    (5, 6),
    (5, 11),
    (6, 12),
    (11, 12),
    (5, 7),
    (6, 8),
    (7, 9),
    (8, 10),
    (11, 13),
    (12, 14),
    (13, 15),
    (14, 16),
];

pub fn check_font(font: &str) -> rusttype::Font<'static> {
    // check then load font

    // ultralytics font path
    let font_path_config = match dirs::config_dir() {
        Some(mut d) => {
            d.push("Ultralytics");
            d.push(font);
            d
        }
        None => panic!("Unsupported operating system. Now support Linux, MacOS, Windows."),
    };

    // current font path
    let font_path_current = std::path::PathBuf::from(font);

    // check font
    let font_path = if font_path_config.exists() {
        font_path_config
    } else if font_path_current.exists() {
        font_path_current
    } else {
        println!("Downloading font...");
        let source_url = "https://ultralytics.com/assets/Arial.ttf";
        let resp = ureq::get(source_url)
            .timeout(std::time::Duration::from_secs(500))
            .call()
            .unwrap_or_else(|err| panic!("> Failed to download font: {source_url}: {err:?}"));

        // read to buffer
        let mut buffer = vec![];
        let total_size = resp
            .header("Content-Length")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap();
        let _reader = resp
            .into_reader()
            .take(total_size)
            .read_to_end(&mut buffer)
            .unwrap();

        // save
        let _path = std::fs::File::create(font).unwrap();
        let mut writer = std::io::BufWriter::new(_path);
        writer.write_all(&buffer).unwrap();
        println!("Font saved at: {:?}", font_path_current.display());
        font_path_current
    };

    // load font
    let buffer = std::fs::read(font_path).unwrap();
    rusttype::Font::try_from_vec(buffer).unwrap()
}