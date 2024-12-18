use crate::range::Range;

pub fn n_natural_sum(n: i64) -> i64 {
  n * (n + 1) / 2
}

pub fn n_natural_sum_between(start: i64, end: i64) -> i64 {
  n_natural_sum(end) - n_natural_sum(start - 1)
}

pub fn n_natural_sum_range(range: Range) -> i64 {
  n_natural_sum_between(range.start, range.end() - 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn between() {
    assert_eq!(n_natural_sum_between(1, 10), 55);
    assert_eq!(n_natural_sum_between(2, 3), 5);
  }

  #[test]
  fn range() {
    assert_eq!(n_natural_sum_range(Range::new(1, 10)), 55);
    assert_eq!(n_natural_sum_range(Range::new(2, 2)), 5);
  }
}
