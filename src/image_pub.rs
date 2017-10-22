extern crate image;

/// CifarImage is inner Struct of CifarDataset.
pub struct CifarImage {
    ///Label number of cifar10.
    pub label: u8,
    ///32x32 RGB image.
    ///
    ///[DynamicImage](https://docs.rs/image/0.17.0/image/enum.DynamicImage.html) is in [image](https://docs.rs/image/0.17.0/image/) crate.
    pub image: self::image::DynamicImage,
}
