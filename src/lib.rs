#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

//!Load Cifar10
//!
//!Cifar10 Simple Loader
//!
//!Use image crate in CifarImage.
//!
//!##Examples
//!
//! Download CIFAR-10 binary version and extract.
//!
//!```
//!# extern crate cifar_10_loader;
//!# use cifar_10_loader::CifarDataset;
//!# fn main()
//!# {
//!//This path is directory of cifar-10-batches-bin.
//!//It's extracted from CIFAR-10 binary version.
//!let cifar10_path = "./cifar-10-batches-bin/";
//!let cifar_dataset = CifarDataset::new(cifar10_path).unwrap();
//! # }
//!```
//#![deny(missing_docs)]


pub use self::image_pub::CifarImage;
pub use self::dataset::CifarDataset;

use self::image_private::CifarImageTrait;

mod image_private;
mod image_pub;
mod dataset;
#[cfg(test)]
mod test;
