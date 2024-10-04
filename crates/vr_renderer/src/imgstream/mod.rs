mod r#static;
mod dynamic;

use image::DynamicImage;

pub use r#static::StaticImageStream;
pub use dynamic::DynamicImageStream;

pub trait ImageStream {
    fn image(&self) -> DynamicImage;
}
