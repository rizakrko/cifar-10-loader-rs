# cifar-10-loader-rs
Crate of loading cifar10 implemented by Rust

## Load Cifar10
Cifar10 Simple Loader

[Documentation](https://docs.rs/cifar_10_loader/)

[crates.io](https://crates.io/crates/cifar_10_loader)

## Examples

Download CIFAR-10 binary version and extract.
```
extern crate cifar_10_loader;
use cifar_10_loader::CifarDataset;

//This path is directory of cifar-10-batches-bin.
//It's extracted from CIFAR-10 binary version.
let cifar10_path = "./cifar-10-batches-bin/";
let cifar_dataset = CifarDataset::new(cifar10_path).unwrap();
```
