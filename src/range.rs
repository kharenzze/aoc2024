#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
  pub start: i64,
  pub len: u64,
}

#[allow(unused)]
impl Range {
  pub fn new(start: i64, len: u64) -> Self {
    Self { start, len }
  }

  pub fn end(&self) -> i64 {
    self.start + self.len as i64
  }

  pub fn contains(&self, value: i64) -> bool {
    value >= self.start && value < self.end()
  }

  pub fn add(&self, value: u64) -> Self {
    Self {
      start: self.start,
      len: self.len + value,
    }
  }

  pub fn add_mut(&mut self, value: u64) {
    self.len += value;
  }

  pub fn take(&self, value: u64) -> Self {
    Self {
      start: self.start,
      len: self.len - value,
    }
  }

  pub fn take_mut(&mut self, value: u64) {
    self.len -= value;
  }
}

impl Default for Range {
  fn default() -> Self {
    Self { start: 0, len: 1 }
  }
}
