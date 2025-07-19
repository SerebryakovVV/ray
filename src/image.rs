use std::{fs::File, io::{self, BufWriter, Write}, path::PathBuf};

use crate::{interval::Interval, vector3::Color};
use crate::camera::Camera;

pub struct Image {
  pub width: u16,
  pub height: u16,
  writer: BufWriter<File>,
  path: PathBuf
}


impl Image {
  pub fn new(width: u16, height: u16, name: &str) -> io::Result<Self> {
    let path = PathBuf::from(format!("C:\\Users\\sereb\\main_files\\coding\\ray\\images\\{}.ppm", name));
    let img_file = File::create(&path)?;
    let mut writer = BufWriter::new(img_file);
    writeln!(writer, "P3\n{width} {height}\n255")?;
    Ok(Self {
      width,
      height,
      writer,
      path
    })
  }

  pub fn draw() {

  }

  pub fn write_color(&mut self, color: Color) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    r = Camera::linear_to_gamma(r);
    g = Camera::linear_to_gamma(g);
    b = Camera::linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i16;
    let gbyte = (256.0 * intensity.clamp(g)) as i16;
    let bbyte = (256.0 * intensity.clamp(b)) as i16;
    let pixel_string = format!("{} {} {}", rbyte, gbyte, bbyte);
    writeln!(self.writer, "{pixel_string}").unwrap_or_else(|e| {
      println!("Error writing a pixel: {}", e);
      std::process::exit(0);
    }); 
  }

  pub fn open_image(self) {
    std::mem::drop(self.writer);
    std::process::Command::new("cmd").args([
      "/C",
      "start",
      self.path.to_str().unwrap()
    ]).status().unwrap_or_else(|e| {
      println!("Error opening the image: {}", e);
      std::process::exit(0);
    });
  }
}