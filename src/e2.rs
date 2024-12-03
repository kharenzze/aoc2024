use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<Vec<i64>>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 2;

struct Game {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
  Up,
  Down,
}

impl Direction {
  fn compute(a: i64) -> Self {
    if a > 0 {
      Direction::Up
    } else {
      Direction::Down
    }
  }
}

const MAX_DIFF: i64 = 3;
const MIN_DIFF: i64 = 1;

impl Game {
  fn check_line(line: &Vec<i64>) -> bool {
    let diffs = Game::compute_diffs(line);
    let main_dir = Direction::compute(diffs[0]);

    //check all elements are same direction
    let all_same_direction = diffs.iter().all(|&d| Direction::compute(d) == main_dir);
    if !all_same_direction {
      return false;
    }

    //check all elements are within the range
    let all_within_range = diffs.iter().all(|&d| {
      let abs_diff = d.abs();
      abs_diff >= MIN_DIFF && abs_diff <= MAX_DIFF
    });
    if !all_within_range {
      return false;
    }

    true
  }

  fn compute_diffs(line: &Vec<i64>) -> Vec<i64> {
    line
      .iter()
      .zip(line.iter().skip(1))
      .map(|(a, b)| b - a)
      .collect()
  }
}

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter
    .map(|l| {
      l.unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
    })
    .collect()
}

fn initial(input: Input) -> Output1 {
  input.iter().filter(|&l| Game::check_line(l)).count() as i64
}

fn extra(input: Input) -> Output2 {
  unimplemented!()
}

pub fn solve(part: usize) {
  let input = read_data(false);
  let score = if part == 1 {
    initial(input)
  } else {
    extra(input)
  };
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 2)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
