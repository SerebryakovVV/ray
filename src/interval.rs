use std::f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
  pub min: f64,
  pub max: f64
}

impl Interval {
  pub fn new(min: f64, max: f64) -> Self {
    Self {min, max}
  }

  pub fn empty() -> Self {
    Self {
      min: INFINITY,
      max: NEG_INFINITY
    }
  }

  pub fn universe() -> Self {
    Self {
      min: NEG_INFINITY, 
      max: INFINITY 
    }
  }

  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }

  pub fn clamp(&self, x: f64) -> f64 {
    if x > self.max {return self.max};
    if x < self.min {return self.min};
    x
  } 
}