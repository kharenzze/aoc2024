use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: i64,
  pub y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bounds {
  min: Point,
  max: Point,
}

#[allow(unused)]
impl Point {
  pub const fn new(x: i64, y: i64) -> Self {
    Self { x, y }
  }

  pub fn is_in_bounds(&self, bounds: &Bounds) -> bool {
    self.x >= bounds.min.x
      && self.x <= bounds.max.x
      && self.y >= bounds.min.y
      && self.y <= bounds.max.y
  }

  pub fn opposite(&self) -> Self {
    -*self
  }

  pub const fn zero() -> Self {
    Self { x: 0, y: 0 }
  }

  pub const fn same_x(self, other: Self) -> bool {
    self.x == other.x
  }

  pub const fn same_y(self, other: Self) -> bool {
    self.y == other.y
  }

  pub const fn same_line(self, other: Self) -> bool {
    self.same_x(other) || self.same_y(other)
  }

  pub const fn normalize(self) -> Self {
    Self {
      x: self.x.signum(),
      y: self.y.signum(),
    }
  }

  pub const fn to_tuple(self) -> (i64, i64) {
    (self.x, self.y)
  }
}

impl Neg for Point {
  type Output = Self;

  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl AddAssign for Point {
  fn add_assign(&mut self, other: Self) {
    *self = *self + other;
  }
}

impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl SubAssign for Point {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other;
  }
}

impl Mul<i64> for Point {
  type Output = Self;

  fn mul(self, other: i64) -> Self {
    Self {
      x: self.x * other,
      y: self.y * other,
    }
  }
}

impl MulAssign<i64> for Point {
  fn mul_assign(&mut self, other: i64) {
    *self = *self * other;
  }
}

impl Bounds {
  pub const fn new(min: Point, max: Point) -> Self {
    Self { min, max }
  }

  pub fn check(&self, point: &Point) -> bool {
    point.is_in_bounds(self)
  }

  pub fn from_dims(x: usize, y: usize) -> Self {
    Self::new(Point::new(0, 0), Point::new(x as i64 - 1, y as i64 - 1))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix2DNavigator {
  pub bounds: Bounds,
  pub current: Point,
  pub direction: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathStatus {
  Full,
  Partial,
}

#[derive(Debug, Clone)]
pub struct Path {
  pub points: Vec<Point>,
  pub status: PathStatus,
}

impl Matrix2DNavigator {
  pub fn next(&mut self) -> Option<Point> {
    let next = self.current + self.direction;
    if self.bounds.check(&next) {
      self.current = next;
      Some(next)
    } else {
      None
    }
  }

  pub fn get_path(&mut self, n: usize) -> Path {
    if n == 0 {
      return Path {
        points: vec![],
        status: PathStatus::Full,
      };
    }
    let steps = n - 1;
    let first = self.current;
    let mut points = Vec::with_capacity(n);
    points.push(first);
    for _ in 0..steps {
      let next = self.next();
      if let Some(next) = next {
        points.push(next);
      } else {
        return Path {
          points,
          status: PathStatus::Partial,
        };
      }
    }
    Path {
      points,
      status: PathStatus::Full,
    }
  }
}
