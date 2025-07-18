use crate::vector3::{Point, Vector3};


pub struct Ray {
  pub origin: Point,
  pub direction: Vector3
}


impl Ray {
  pub fn new(origin: Point, direction: Vector3) -> Self {
    Self {origin, direction}
  }

  pub fn at(&self, t: f64) -> Point {
    self.origin + t * self.direction
  }
}