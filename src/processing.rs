extern crate image;

use image::imageops::FilterType;
use image::{GenericImageView, ImageFormat};
use std::fmt;
use std::path::Path;
use std::time::{Duration, Instant};

pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

pub fn run_conversion(
    source_image_path: &String,
    destination_image_path: &String,
    image_size: ImageSize,
) {
    let im = image::open(&Path::new(&source_image_path)).unwrap();
    println!("dimensions {:?}", im.dimensions());
    let timer = Instant::now();
    let resized_img = im.resize(image_size.width, image_size.height, FilterType::Nearest);
    println!(
        "Thumbnailed to {}x{} in {}",
        image_size.width,
        image_size.height,
        Elapsed::from(&timer)
    );
    // Write the contents of this image using extension guessing.
    resized_img
        .save(&Path::new(&destination_image_path))
        .unwrap();
}
