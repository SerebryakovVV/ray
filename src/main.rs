mod image;
mod vector3;


use image::Image;
use vector3::Color;


// look into implementing Deref trait and newtype pattern

const WIDTH: u16 = 256;
const HEIGHT: u16 = 256;

fn main() {
  let name = std::env::args().collect::<Vec<String>>()
                             .into_iter()
                             .skip(1)
                             .next()
                             .unwrap_or_else(||{
                                println!("No name provided");
                                std::process::exit(0);});
  let mut img = Image::new(WIDTH, HEIGHT, &name).unwrap_or_else(|e| {
    println!("Error occured while creating a file: {}", e);
    std::process::exit(0);
  });
  for row in 0..img.height {
    println!("Rows remain: {}", img.height - row);
    for col in 0..img.width {
      img.write_color(
        Color::new(
          col as f64 / (img.width - 1) as f64,
          row as f64 / (img.height - 1) as f64,
          0.0
        )
      );
    }
  }

  img.open_image();
}