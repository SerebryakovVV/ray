use std::f64::INFINITY;
use rand::rngs::ThreadRng;
use rand::Rng;
use crate::hittable::{Hittable, HittableList};
use crate::image::Image;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector3::{Color, Point, Vector3};
use crate::{
  IMAGE_WIDTH,
  IMAGE_HEIGHT,
  VIEWPORT_WIDTH,
  VIEWPORT_HEIGHT,
};


pub struct Camera {
  pub focal_length: f64,
  pub camera_center: Vector3,
  pub viewport_u: Vector3,
  pub viewport_v: Vector3,
  pub pixel_delta_u: Vector3,
  pub pixel_delta_v: Vector3,
  pub viewport_upper_left: Vector3,
  pub pixel00_loc: Vector3,
  pub img: Image,
  pub samples_per_pixel: i32,
  pub pixel_samples_scale: f64,
  pub rng: ThreadRng
}


impl Camera {
  pub fn new(img: Image) -> Self {
    let focal_length = 1.0;
    let camera_center = Point::new(0.0, 0.0, 0.0);
    let viewport_u = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;
    let viewport_upper_left = camera_center 
                            - Vector3::new(0.0, 0.0, focal_length) 
                            - viewport_u / 2.0 
                            - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    let samples_per_pixel = 100;
    let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
    let rng = rand::rng();
  // rnd.random::<f64>();
    Self { 
      focal_length,
      camera_center,
      viewport_u,
      viewport_v,
      pixel_delta_u,
      pixel_delta_v,
      viewport_upper_left,
      pixel00_loc,
      img,
      samples_per_pixel,
      pixel_samples_scale,
      rng
    }
  }


  pub fn ray_color<T: Hittable>(&self, ray: &Ray, world: &T) -> Color {
    if let Some(rec) = world.hit(ray, Interval::new(0.0, INFINITY)) {
      let direction = Vector3::random_on_hemishpere(rec.normal);
      return 0.5 * self.ray_color(&Ray::new(rec.p, direction), world);
    }
    let unit_direction = Vector3::unit_vector(&ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) +  a * Color::new(0.5, 0.7, 1.0)
  }

  pub fn render(&mut self, world: &HittableList) {
    for row in 0..self.img.height {
      println!("Rows remain: {}", self.img.height - row);
      for col in 0..self.img.width {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for s in 0..self.samples_per_pixel {
          let ray = self.get_ray(col, row);
          color += self.ray_color(&ray, world);
        };
        self.img.write_color(self.pixel_samples_scale * color);      
      }
    }
  }

  pub fn get_ray(&mut self, col: u16, row: u16) -> Ray {
    let offset = self.sample_square();
    let pixel_sample = self.pixel00_loc
                   + ((col as f64 + offset.x) * self.pixel_delta_u)
                   + ((row as f64 + offset.y) * self.pixel_delta_v);
    let ray_origin = self.camera_center;
    let ray_direction = pixel_sample - ray_origin;
    Ray::new(ray_origin, ray_direction)
  }

  pub fn sample_square(&mut self) -> Vector3 {
    Vector3::new(
      self.rng.random::<f64>() - 0.5,
      self.rng.random::<f64>() - 0.5,
      0.0
    )
  } 
}