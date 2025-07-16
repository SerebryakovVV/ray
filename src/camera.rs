use crate::vector3::{Vector3, Point};
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
}


impl Camera {
  pub fn new() -> Self {
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
    Self { 
      focal_length,
      camera_center,
      viewport_u,
      viewport_v,
      pixel_delta_u,
      pixel_delta_v,
      viewport_upper_left,
      pixel00_loc 
    }
  }
}