extern crate image;
extern crate itertools;
extern crate rand;
extern crate walkdir;

pub struct CifarImage {
    pub label: u8,
    pub image: image::DynamicImage,
}

impl CifarImage {
    pub fn new(bytes: &[u8]) -> Result<Self, ::std::io::Error> {
        use std::io::Read;
        use std::mem;
        use self::image::GenericImage;
        use self::itertools::multizip;
        use self::itertools::Itertools;
        let bytes: &mut &[u8] = &mut bytes.as_ref();
        let label: u8 = unsafe {
            let label: &mut [u8; 1] = &mut mem::uninitialized();
            bytes.read_exact(label)?;
            *label.get_unchecked(0)
        };
        let img = unsafe {
            let mut img = image::DynamicImage::new_rgb8(32, 32);
            let red: &mut [u8; 1024] = &mut mem::uninitialized();
            let green: &mut [u8; 1024] = &mut mem::uninitialized();
            let blue: &mut [u8; 1024] = &mut mem::uninitialized();
            bytes.read_exact(red)?;
            bytes.read_exact(green)?;
            bytes.read_exact(blue)?;
            multizip((
                (0..32).cartesian_product(0..32),
                red.iter(),
                green.iter(),
                blue.iter(),
            )).for_each(|((y, x), r, g, b)| {
                let mut pixel: image::Rgba<u8> = mem::uninitialized();
                pixel.data = [*r, *g, *b, 255];
                img.unsafe_put_pixel(x, y, pixel);
            });
            img
        };
        Ok(CifarImage {
            label: label,
            image: img,
        })
    }
}
