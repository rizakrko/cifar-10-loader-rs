extern crate curl;
extern crate find_folder;
extern crate image;
extern crate rand;

#[test]
fn test_cifar_10_loader_rs() {
    let path = get_cifar_10();
    let cifar_dataset = super::CifarDataset::new(path).unwrap();
    let rng = &mut rand::thread_rng();
    test_load_data(&cifar_dataset);
    test_output(&cifar_dataset, rng).unwrap();
}
fn get_cifar_10() -> ::std::path::PathBuf {
    use self::find_folder::Search;
    let dir = Search::Kids(3).for_folder("cifar-10-batches-bin");
    let tar = Search::Kids(1).for_folder("cifar-10-binary.tar.gz");
    match (dir, tar) {
        (Ok(path), _) => path,
        (Err(_), Ok(path)) => extract_cifar_10(path),
        (Err(_), Err(_)) => {
            let path = download_cifar_10();
            extract_cifar_10(path)
        }
    }
}
fn download_cifar_10() -> ::std::path::PathBuf {
    let path = "./cifar-10-binary.tar.gz";
    use std::fs;
    use std::io::{BufWriter, Write};
    use self::curl::easy::Easy;
    let mut easy = Easy::new();
    let mut file = BufWriter::new(fs::File::create(path).unwrap());
    easy.url("https://www.cs.toronto.edu/~kriz/cifar-10-binary.tar.gz")
        .unwrap();
    easy.write_function(move |data| Ok(file.write(data).unwrap()))
        .unwrap();
    easy.perform().unwrap();
    ::std::path::PathBuf::from(path)
}
fn extract_cifar_10(path: ::std::path::PathBuf) -> ::std::path::PathBuf {
    use std::process::Command;
    let path = &path.to_string_lossy();
    if cfg!(target_os = "windows") {
        panic!("Windows Can Not Extract tar.gz");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(("tar xf ".to_string() + path))
            .output()
            .unwrap();
    };
    ::std::path::PathBuf::from("./cifar-10-batches-bin")
}
fn for_test_get_image_from_train_save(
    data_set: &super::CifarDataset,
    rng: &mut rand::ThreadRng,
) -> Result<(), String> {
    use self::rand::Rng;
    let fout = &mut ::std::fs::File::create(&::std::path::Path::new("train.jpeg"))
        .map_err(|err| err.to_string())?;
    let nth: &usize = &rng.gen_range(0, data_set.train_count);
    let data: &super::CifarImage = &data_set.train_dataset[*nth];
    data.image
        .resize(500, 500, image::FilterType::Lanczos3)
        .save(fout, image::JPEG)
        .map_err(|err| err.to_string())?;
    println!(
        "From Train No.{} {}",
        nth,
        data_set.labels[data.label as usize]
    );
    Ok(())
}
fn for_test_get_image_from_test_save(
    data_set: &super::CifarDataset,
    rng: &mut rand::ThreadRng,
) -> Result<(), String> {
    use self::rand::Rng;
    let fout = &mut ::std::fs::File::create(&::std::path::Path::new("test.jpeg"))
        .map_err(|err| err.to_string())?;
    let nth: &usize = &rng.gen_range(0, data_set.test_count);
    let data: &super::CifarImage = &data_set.test_dataset[*nth];
    data.image
        .resize(500, 500, image::FilterType::Lanczos3)
        .save(fout, image::JPEG)
        .map_err(|err| err.to_string())?;
    println!(
        "From test No.{} {}",
        nth,
        data_set.labels[data.label as usize]
    );
    Ok(())
}
fn test_load_data(data_set: &super::CifarDataset) {
    let labels = vec![
        "airplane",
        "automobile",
        "bird",
        "cat",
        "deer",
        "dog",
        "frog",
        "horse",
        "ship",
        "truck",
    ];
    assert_eq!(
        labels,
        data_set.labels,
        "we are testing labels loaded properly.\n expect:{:?} \n result {:?}",
        labels,
        data_set.labels
    );
    assert_eq!(
        10_000,
        data_set.test_count,
        "we are testing test_data loaded properly.\n expect_count:{} \n result_count {:?}",
        10_000,
        data_set.test_count
    );
    assert_eq!(
        50_000,
        data_set.train_count,
        "we are testing train_data loaded properly.\n expect_count:{} \n result_count {:?}",
        50_000,
        data_set.train_count
    );
}
fn test_output(data_set: &super::CifarDataset, rng: &mut rand::ThreadRng) -> Result<(), String> {
    for_test_get_image_from_train_save(data_set, rng)?;
    for_test_get_image_from_test_save(data_set, rng)?;
    Ok(())
}
