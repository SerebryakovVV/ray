// i have a base struct of vector3 with x,y,z. Then, i guess, i have 3 newtypes
// for point, ray and color. Then i have vector3base trait, that has all the 
// methods that could be on any of these newtypes, and also these methods have 
// default implementations. Well, whats next, then i have some methods that are 
// unique for each newtype and i implement them separately? in impl newtypename 
// {} i supose. And then i will also have operator overloading 


// Dot product = how aligned two vectors are (a number).
// Cross product = perpendicular vector that encodes area and orientation.
// Unit vector = direction vector with length 1, stripped of scale.

use std::{
  fmt::Display, 
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub}
};



pub type Color = Vector3;
pub type Ray = Vector3;
pub type Point = Vector3;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}


impl Vector3 {

  pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
    Self {
      x: x.into(), 
      y: y.into(),
      z: z.into()
    }
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.x * self.x +
    self.y * self.y +
    self.z * self.z
  }

  pub fn dot(left: Self, right: Self) -> f64 {
    left.x * right.x +
    left.y * right.y +
    left.z * right.z
  }

  pub fn cross(left: Self, right: Self) -> Self {
    Self {
      x: left.y * right.z - left.z * right.y,
      y: left.z * right.x - left.x * right.z,
      z: left.x * right.y - left.y * right.x
    }
  }

  pub fn unit_vector(v: &Self) -> Self {
    *v / v.length()
  }
}


impl Display for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
  }
}


impl Add for Vector3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}


impl AddAssign for Vector3 {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}


impl Neg for Vector3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}


impl Sub for Vector3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}


impl Div<f64> for Vector3 {
  type Output = Self;
  fn div(self, rhs: f64) -> Self::Output {
      (1.0 / rhs) * self
  }
}


impl DivAssign<f64> for Vector3 {
  fn div_assign(&mut self, rhs: f64) {
      let temp = 1.0 / rhs;
      self.x *= temp;
      self.y *= temp;
      self.z *= temp;
  }
}


impl Mul for Vector3 {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
      Self {
        x: self.x * rhs.x,
        y: self.y * rhs.y,
        z: self.z * rhs.z,
      }
  }
}

impl MulAssign for Vector3 {
  fn mul_assign(&mut self, rhs: Self) {
      self.x *= rhs.x;
      self.y *= rhs.y;
      self.z *= rhs.z;
  }
}


impl Mul<f64> for Vector3 {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}


impl Mul<Vector3> for f64 {
  type Output = Vector3;
  fn mul(self, rhs: Vector3) -> Self::Output {
      rhs * self
  }
}




#[allow(dead_code)]
#[cfg(test)]
mod vec_tests {
  use super::*;
  
  #[test]
  fn test_add() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let b = Vector3 {x: 5.0, y: 7.0, z: 9.0};
    assert_eq!(a + b, Vector3 {x: 6.0, y: 9.0, z: 12.0});
  }

  #[test]
  fn test_add_assign() {
    let mut a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    a += Vector3 {x: 6.0, y: 9.0, z: 12.0};
    assert_eq!(a, Vector3 {x: 7.0, y: 11.0, z: 15.0});
  }

  #[test]
  fn test_neg() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    assert_eq!(-a, Vector3 {x: -1.0, y: -2.0, z: -3.0});
  }

  #[test]
  fn test_sub() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let b = Vector3 {x: 5.0, y: 7.0, z: 9.0};
    assert_eq!(b - a, Vector3 {x: 4.0, y: 5.0, z: 6.0})
  }

  #[test]
  fn test_div_64() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let b = a / 2.0;
    assert!((b.x - 0.5).abs() < 0.000001);
    assert!((b.y - 1.0).abs() < 0.000001);
    assert!((b.z - 1.5).abs() < 0.000001);
  }

  #[test]
  fn test_div_assign_64() {
    let mut a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    a /= 10.0;
    assert!((a.x - 0.1).abs() < 0.000001);
    assert!((a.y - 0.2).abs() < 0.000001);
    assert!((a.z - 0.3).abs() < 0.000001);
  }

  #[test]
  fn test_mul() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let b = Vector3 {x: 5.0, y: 7.0, z: 9.0};
    assert_eq!(a * b, Vector3 {x: 5.0, y: 14.0, z: 27.0});
  }

  #[test]
  fn test_mul_assign() {
    let mut a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let b = Vector3 {x: 5.0, y: 7.0, z: 9.0};
    a *= b;
    assert_eq!(a, Vector3 {x: 5.0, y: 14.0, z: 27.0});
  }

  #[test]
  fn test_mul_64() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    assert_eq!(a * 10.0, Vector3 {x: 10.0, y: 20.0, z: 30.0});
  }

  #[test]
  fn test_mul_vector3() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    assert_eq!(10.0 * a, Vector3 {x: 10.0, y: 20.0, z: 30.0});
  }

  #[test]
  fn test_length_squared() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    assert_eq!(a.length_squared(), 14.0);
  }

  #[test]
  fn test_length() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let expected = 14.0f64.sqrt();
    let actual = a.length();
    assert!((expected - actual).abs() < 0.000001);
  }

  #[test]
  fn test_display() {
    let a = Vector3 {x: 1.1, y: 2.0, z: 3.0};
    println!("{}", a);
    assert_eq!(format!("{}", a), "[1.1, 2, 3]");
  }

  #[test]
  fn test_dot() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let b = Vector3 {x: 5.0, y: 7.0, z: 9.0};
    assert_eq!(
      Vector3::dot(a, b),
      1.0 * 5.0 + 2.0 * 7.0 + 3.0 * 9.0
    )
  }

  #[test]
  fn test_cross() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let b = Vector3 {x: 3.0, y: 4.0, z: 5.0};
    assert_eq!(
      Vector3::cross(a, b),
      Vector3 {x: -2.0, y: 4.0, z: -2.0}
    )
  }

  #[test]
  fn test_unit_vector() {
    let a = Vector3 {x: 1.0, y: 2.0, z: 3.0};
    let u = Vector3::unit_vector(&a);
    assert!((u.x - 0.2672612).abs() < 0.000001);
    assert!((u.y - 0.5345224).abs() < 0.000001);
    assert!((u.z - 0.8017837).abs() < 0.000001);
  }
}