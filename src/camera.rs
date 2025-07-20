use std::f64::INFINITY;
use rand::rngs::ThreadRng;
use rand::Rng;
use crate::hittable::{Hittable, HittableList};
use crate::image::Image;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::{Color, Point, Vector3};
use crate::{
  IMAGE_WIDTH,
  IMAGE_HEIGHT,
  degrees_to_radians
};


pub struct Camera {
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
  pub rng: ThreadRng,
  pub max_depth: i32,
  pub vfov: f64,
  pub u: Vector3,
  pub v: Vector3,
  pub w: Vector3,
  defocus_disk_u: Vector3,
  defocus_disk_v: Vector3,
  pub defocus_angle: f64,
  pub focus_dist: f64,
}


impl Camera {
  pub fn new(img: Image) -> Self {
    let vfov = 90.0;
    let lookfrom = Point::new(0.0, 0.0, 0.4);
    let lookat = Point::new(0.0, 0.0, -1.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let w = Vector3::unit_vector(&(lookfrom - lookat));
    let u = Vector3::unit_vector(&Vector3::cross(vup, w));
    let v = Vector3::cross(w, u);
    let theta = degrees_to_radians(vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h * focus_dist;
    let viewport_widht = viewport_height * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let camera_center = lookfrom;
    let viewport_u = viewport_widht * u;
    let viewport_v = viewport_height * -v;
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;
    let viewport_upper_left = camera_center 
                            - focus_dist * w 
                            - viewport_u / 2.0 
                            - viewport_v / 2.0;
    let defocus_radius = focus_dist * (degrees_to_radians(defocus_angle / 2.0)).tan();
    let defocus_disk_u = u * defocus_radius;
    let defocus_disk_v = v * defocus_radius;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    let samples_per_pixel = 500;
    let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
    let rng = rand::rng();
    let max_depth = 50;
    Self { 
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
      rng,
      max_depth,
      vfov,
      u, 
      v,
      w,
      defocus_disk_u,
      defocus_disk_v,
      defocus_angle,
      focus_dist
    }
  }

  pub fn ray_color<T: Hittable>(&self, ray: &Ray, depth: i32, world: &T) -> Color {
    if depth <= 0 {
      return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(ray, Interval::new(0.001, INFINITY)) {
      if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
        return attenuation * self.ray_color(&scattered, depth - 1, world);
      } else {
        return Color::new(0.0, 0.0, 0.0)
      }
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
          color += self.ray_color(&ray, self.max_depth, world);
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
    let ray_origin = if self.defocus_angle <= 0.0 {self.camera_center} else {self.defocus_disk_sample()};
    let ray_direction = pixel_sample - ray_origin;
    Ray::new(ray_origin, ray_direction)
  }

  pub fn defocus_disk_sample(&self) -> Point {
    let p = Vector3::random_in_unit_disk();
    self.camera_center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
  }

  pub fn sample_square(&mut self) -> Vector3 {
    Vector3::new(
      self.rng.random::<f64>() - 0.5,
      self.rng.random::<f64>() - 0.5,
      0.0
    )
  } 

  pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
      linear_component.sqrt()
    } else {
      0.0
    }
  }
}