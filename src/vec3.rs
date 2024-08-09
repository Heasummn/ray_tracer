use std::cmp;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::util::Interval;

// Vec3 class for storing data, such as colors or location

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {

  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn zeros() -> Vec3 {
    Vec3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
  pub fn ones() -> Vec3 {
    Vec3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    }
  }
  pub fn of(val: f64) -> Vec3 {
    Vec3 {
      x: val,
      y: val,
      z: val,
    }
  }

  pub fn norm(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn length(&self) -> f64 {
    self.norm().sqrt()
  }

  pub fn as_unit(&self) -> Vec3 {
    let length = self.length();

    return Vec3 {
      x: self.x / length,
      y: self.y / length,
      z: self.z / length,
    };
  }

  pub fn dot(&self, other: &Vec3) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: &Vec3) -> Vec3 {
    Vec3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }
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

impl Mul<Vec3> for Vec3 {
  type Output = f64;

  fn mul(self, other: Vec3) -> f64 {
    return self.x * other.x + self.y * other.y + self.z * other.z;
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    return Vec3 {
      x: self * other.x,
      y: self * other.y,
      z: self * other.z,
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

impl Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Vec3 {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
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


// assumes vec is in range 0-1
#[cfg(feature = "image")]
impl From<Vec3> for image::Rgb<u8> {
  fn from(vec: Vec3) -> Self {
    const INTENSITY: Interval = Interval {min: 0.0, max: 1.0};
    let r = (INTENSITY.clamp(vec.x) * 255.0) as u8;
    let g = (INTENSITY.clamp(vec.y) * 255.0) as u8;
    let b = (INTENSITY.clamp(vec.z) * 255.0) as u8;
    return image::Rgb([r, g, b]);
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

// This test is messy as floats are not perfect and you cannot assume perfect precision, especially with sqrt
#[test]
fn can_make_unit() {
  let op1 = Vec3 {
    x: 1.0,
    y: 3.0,
    z: 18.0,
  };

  assert_eq!(op1.as_unit().norm().round(), 1.0)
}

#[test]
fn can_dot() {
  let op1 = Vec3 {
    x: 9.0,
    y: 2.0,
    z: 7.0,
  };
  let op2 = Vec3 {
    x: 4.0,
    y: 8.0,
    z: 10.0,
  };

  assert_eq!(op1.dot(&op2), 122.0)
}

#[test]
fn can_cross() {
  let op1 = Vec3 {
    x: 2.0,
    y: 3.0,
    z: 4.0,
  };

  let op2 = Vec3 {
    x: 5.0,
    y: 6.0,
    z: 7.0,
  };

  assert_eq!(
    op1.cross(&op2),
    Vec3 {
      x: -3.0,
      y: 6.0,
      z: -3.0
    }
  )
}
