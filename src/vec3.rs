use std::cmp;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// Vec3 class for storing data, such as colors or location

#[derive(Default)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    return Vec3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    };
  }
}

impl Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, other: Vec3) -> Vec3 {
    return Vec3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    };
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, other: f64) -> Vec3 {
    return Vec3 {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    };
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, other: f64) -> Vec3 {
    return Vec3 {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other,
    };
  }
}

impl cmp::PartialEq for Vec3 {
  fn eq(&self, other: &Vec3) -> bool {
    return self.x == other.x && self.y == other.y && self.z == other.z;
  }
}

impl fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "({}, {}, {})", self.x, self.y, self.z);
  }
}

#[cfg(feature = "image")]
impl From<Vec3> for image::Rgb<u8> {
  fn from(vec: Vec3) -> Self {
    return image::Rgb([vec.x as u8, vec.y as u8, vec.z as u8]);
  }
}

#[test]
fn can_compare_equality() {
  assert!(
    Vec3 {
      x: 3.0,
      y: 4.0,
      z: 1.0
    } == Vec3 {
      x: 3.0,
      y: 4.0,
      z: 1.0
    }
  );
}

#[test]
fn can_add() {
  let op1 = Vec3 {
    x: 1.0,
    y: 2.0,
    z: 1.0,
  };
  let op2 = Vec3 {
    x: 3.0,
    y: 2.0,
    z: 3.0,
  };
  assert_eq!(
    op1 + op2,
    Vec3 {
      x: 4.0,
      y: 4.0,
      z: 4.0
    }
  )
}

#[test]
fn can_sub() {
  let op1 = Vec3 {
    x: 5.0,
    y: 3.0,
    z: 4.0,
  };
  let op2 = Vec3 {
    x: 2.0,
    y: 3.0,
    z: 2.0,
  };

  assert_eq!(
    op1 - op2,
    Vec3 {
      x: 3.0,
      y: 0.0,
      z: 2.0
    }
  );
}

#[test]
fn can_mul_scalar() {
  let op1 = Vec3 {
    x: 1.0,
    y: 0.5,
    z: 3.0,
  };
  let op2 = 2.0;

  assert_eq!(
    op1 * op2,
    Vec3 {
      x: 2.0,
      y: 1.0,
      z: 6.0
    }
  )
}

#[test]
fn can_div_scalar() {
  let op1 = Vec3 {
    x: 6.0,
    y: 2.0,
    z: 4.0,
  };
  let op2 = 2.0;
  assert_eq!(
    op1 / op2,
    Vec3 {
      x: 3.0,
      y: 1.0,
      z: 2.0
    }
  )
}
