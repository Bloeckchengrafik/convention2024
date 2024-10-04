use std::io::Cursor;
use image::DynamicImage;
use crate::imgstream::ImageStream;
use image::io::Reader as ImageReader;


pub struct StaticImageStream {
    image: DynamicImage,
}

impl StaticImageStream {
    pub fn new(image: &[u8]) -> Box<Self> {
        Box::new(Self {
            image: load_image(image)
        })
    }
}

impl ImageStream for StaticImageStream {
    fn image(&self) -> DynamicImage {
        self.image.clone()
    }
}

fn load_image(bytes: &[u8]) -> DynamicImage {
    ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap()
}
