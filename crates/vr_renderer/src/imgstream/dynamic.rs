use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::thread;
use futures_util::StreamExt;
use image::{DynamicImage, io::Reader as ImageReader};
use tokio::runtime::Runtime;
use crate::imgstream::ImageStream;

pub struct DynamicImageStream {
    image: Arc<Mutex<Option<DynamicImage>>>,
}

impl DynamicImageStream {
    async fn fetch_mjpeg_stream(url: &str, image: Arc<Mutex<Option<DynamicImage>>>) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err("Failed to connect to stream".into());
        }

        let mut stream = response.bytes_stream();
        let mut buffer: Vec<u8> = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.extend_from_slice(&chunk);

            if let Some(start) = find_jpeg_start(&buffer) {
                if let Some(end) = find_jpeg_end(&buffer, start) {
                    let jpeg_data = &buffer[start..=end];

                    if let Ok(img) = ImageReader::new(Cursor::new(jpeg_data))
                        .with_guessed_format()?
                        .decode() {
                        let mut current_image = image.lock().unwrap();
                        *current_image = Some(img);
                    }

                    buffer.drain(..=end);
                }
            }
        }

        Ok(())
    }


    pub fn new(source: &str) -> Box<Arc<Self>> {
        let image = Arc::new(Mutex::new(None));
        let stream = Arc::new(Self {
            image: Arc::clone(&image),
        });

        let url = source.to_string();
        let image_clone = Arc::clone(&image);

        thread::spawn(move || {
            // Create a new Tokio runtime for this thread
            let rt = Runtime::new().unwrap();

            // Run the MJPEG stream fetching task on this runtime
            rt.block_on(async move {
                if let Err(e) = Self::fetch_mjpeg_stream(&url, image_clone).await {
                    eprintln!("Error fetching MJPEG stream: {}", e);
                }
            });
        });

        Box::new(stream)
    }
}

impl ImageStream for DynamicImageStream {
    fn image(&self) -> DynamicImage {
        let guard = self.image.lock().unwrap();
        if let Some(ref img) = *guard {
            img.clone().resize_to_fill(640, 480, image::imageops::FilterType::Nearest)
        } else {
            DynamicImage::new_rgba8(1, 1).resize_to_fill(640, 480, image::imageops::FilterType::Nearest)
        }
    }
}

// Function to find the start of a JPEG image (FFD8)
fn find_jpeg_start(buffer: &[u8]) -> Option<usize> {
    buffer.windows(2).position(|w| w == [0xFF, 0xD8])  // JPEG Start marker FFD8
}

// Function to find the end of a JPEG image (FFD9), starting from the start position
fn find_jpeg_end(buffer: &[u8], start: usize) -> Option<usize> {
    buffer[start..].windows(2).position(|w| w == [0xFF, 0xD9])  // JPEG End marker FFD9
        .map(|pos| start + pos + 1)  // Adjust for window size
}
