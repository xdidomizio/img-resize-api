extern crate image;

use image::imageops::FilterType;
use image::{GenericImageView, ImageFormat};
use std::env;
use std::fmt;
use std::path::Path;
use std::time::{Duration, Instant};

struct Elapsed(Duration);

struct ImageSize {
    width: u32,
    height: u32,
}

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

fn main() {
    let (from, into, width, height) = if env::args_os().count() == 5 {
        (
            env::args_os().nth(1).unwrap().into_string().unwrap(),
            env::args_os().nth(2).unwrap().into_string().unwrap(),
            env::args_os().nth(3).unwrap().into_string().unwrap(),
            env::args_os().nth(4).unwrap().into_string().unwrap(),
        )
    } else {
        println!("Please enter a from and into path.");
        std::process::exit(1);
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let image_size = ImageSize {
        width: width.parse().unwrap(),
        height: height.parse().unwrap(),
    };
    run_conversion(&from, &into, image_size);
}

fn run_conversion(
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
