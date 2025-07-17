mod image;
mod vector3;
mod ray;
mod camera;
mod hittable;
mod sphere;

use image::Image;
use vector3::{Color, Point, Vector3};
use ray::Ray;
use camera::Camera;



// look into implementing Deref trait and newtype pattern

// the direction of the vector is just the ammount of movement in each coordinate direction
// normalization is the same but its mapped onto 0-1 range


// pixel spacing - distance between pixels

const ASPECT_RATIO: f64 = 16.0 / 9.0; 
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = {
  let temp = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
  if temp < 1 {1} else {temp}
};
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);




fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> f64 {
  let oc = center - ray.origin;
  let a = ray.direction.length_squared();
  let h = Vector3::dot(ray.direction, oc);
  let c = oc.length_squared() - radius * radius;
  let discriminant = h * h - a * c;
  if discriminant < 0.0 {
    -1.0
  } else {
    (h - discriminant.sqrt()) / a
  }
}



fn ray_color(ray: Ray) -> Color {
  let t = hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &ray);
  if t > 0.0 {
    let n = Vector3::unit_vector(&(ray.at(t) - Vector3::new(0.0, 0.0, -1.0)));
    return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
  }
  let unit_direction = Vector3::unit_vector(&ray.direction);
  let a = 0.5 * (unit_direction.y + 1.0);
  (1.0 - a) * Color::new(1.0, 1.0, 1.0) +  a * Color::new(0.5, 0.7, 1.0)
}




fn main() {
  let name = std::env::args().collect::<Vec<String>>()
                             .into_iter()
                             .skip(1)
                             .next()
                             .unwrap_or_else(||{
                                println!("No name provided");
                                std::process::exit(0);});
  let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, &name).unwrap_or_else(|e| {
    println!("Error occured while creating a file: {}", e);
    std::process::exit(0);
  });
  let cam = Camera::new();  
  for row in 0..img.height {
    println!("Rows remain: {}", img.height - row);
    for col in 0..img.width {
      let pixel_center = cam.pixel00_loc + (col as f64 * cam.pixel_delta_u) + (row as f64 * cam.pixel_delta_v);  // implement safe mul for u16, or into trait
      let ray_direction = pixel_center - cam.camera_center;
      let r = Ray::new(cam.camera_center, ray_direction);
      let color = ray_color(r);
      img.write_color(color);
    }
  }
  img.open_image();
}