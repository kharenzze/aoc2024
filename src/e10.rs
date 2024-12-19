use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::point::{Bounds, Matrix2DNavigator, Point};

type Input = Vec<Vec<i64>>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 10;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter
    .map(|l| {
      l.unwrap()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as i64)
        .collect()
    })
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  const fn to_point(&self) -> Point {
    match self {
      Direction::Up => Point::new(0, -1),
      Direction::Down => Point::new(0, 1),
      Direction::Left => Point::new(-1, 0),
      Direction::Right => Point::new(1, 0),
    }
  }
}

static DIRECTIONS: [Direction; 4] = [
  Direction::Up,
  Direction::Down,
  Direction::Left,
  Direction::Right,
];

static MAX_VALUE: i64 = 9;

fn initial(input: Input) -> Output1 {
  let bounds = Bounds::from_dims(input[0].len(), input.len());
  let mut zeros: Vec<Point> = vec![];
  for y in 0..input.len() {
    for x in 0..input[y].len() {
      if input[y][x] == 0 {
        zeros.push(Point::new(x as i64, y as i64));
      }
    }
  }

  let mut navigator = Matrix2DNavigator {
    bounds,
    current: Point::default(),
    direction: Direction::Right.to_point(),
  };

  let mut score: i64 = 0;
  for z in zeros {
    let mut open = vec![z];
    let mut top_positions: HashSet<Point> = HashSet::new();
    while let Some(p) = open.pop() {
      let current_value = input[p.y as usize][p.x as usize];
      navigator.current = p;
      let expected_next_value = current_value + 1;
      for d in DIRECTIONS {
        navigator.direction = d.to_point();
        let Some(next) = navigator.read_next() else {
          continue;
        };
        let next_value = input[next.y as usize][next.x as usize];
        if next_value == expected_next_value {
          if next_value == MAX_VALUE {
            top_positions.insert(next);
          } else {
            open.push(next);
          }
        }
      }
    }
    score += top_positions.len() as i64;
  }

  score
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
    assert_eq!(score, 36)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
