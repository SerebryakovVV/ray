mod image;


use image::Image;

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
    for col in 0..img.width {
      let r = col as f64 / (img.width - 1) as f64;
      let g = row as f64 / (img.height - 1) as f64;
      let b = 0.0f64;
      let ir = (255.999 * r) as i16;
      let ig = (255.999 * g) as i16;
      let ib = (255.999 * b) as i16;
      let pixel_string = format!("{} {} {}", ir, ig, ib);
      img.write(&pixel_string);
    }
  }

  img.open_image();
}