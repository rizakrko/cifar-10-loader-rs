extern crate image;

pub use self::image::DynamicImage;
/// CifarImage is inner Struct of CifarDataset.
pub struct CifarImage {
    ///Label number of cifar10.
    pub label: u8,
    ///32x32 RGB image.
    ///
    ///DynamicImage is in image crate.
    pub image: self::image::DynamicImage,
}
