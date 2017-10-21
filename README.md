# cifar-10-loader-rs
Crate of loading cifar10 implemented by Rust

## Load Cifar10
Cifar10 Simple Loader

[Documentation](https://29rou.github.io/cifar-10-loader-rs/)

## Examples

Download CIFAR-10 binary version and extact.
```
extern crate cifar_10_loader;
use cifar_10_loader::CifarDataset;

//This path is directory of cifar-10-batches-bin.
//It's extracted from CIFAR-10 binary version.
let cifar10_path = "./cifar-10-batches-bin/";
let cifar_dataset = CifarDataset::new(cifar10_path).unwrap();
```
