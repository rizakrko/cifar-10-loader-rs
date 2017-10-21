
//!Load Cifar10
//!
//!Cifar10 Simple Loader
//!
//!##Examples
//!
//! Download CIFAR-10 binary version and extact.
//!
//!```
//!extern crate cifar_10_loader;
//!use cifar_10_loader::CifarDataset;
//!
//!//This path is directory of cifar-10-batches-bin.
//!//It's extracted from CIFAR-10 binary version.
//!let cifar10_path = "./cifar-10-batches-bin/";
//!let cifar_dataset = CifarDataset::new(cifar10_path).unwrap();
//!```
pub extern crate image;

pub use self::image_pub::CifarImage;
pub use self::dataset::CifarDataset;

use self::image_private::CifarImageTrait;

mod image_private;
mod image_pub;
mod dataset;
