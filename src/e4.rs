use crate::point::{Bounds, Matrix2DNavigator, PathStatus, Point};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<Vec<char>>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 4;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap().chars().collect()).collect()
}

static DIRECTIONS: [Point; 8] = [
  Point::new(0, 1),
  Point::new(1, 0),
  Point::new(0, -1),
  Point::new(-1, 0),
  Point::new(1, 1),
  Point::new(-1, -1),
  Point::new(1, -1),
  Point::new(-1, 1),
];

const TARGET: &str = "XMAS";

fn initial(input: Input) -> Output1 {
  let mut score: Output1 = 0;
  let bounds = Bounds::from_dims(input.len(), input[0].len());
  for (i, line) in input.iter().enumerate() {
    for (j, &c) in line.iter().enumerate() {
      if c != 'X' {
        continue;
      }
      let current = Point::new(i as i64, j as i64);
      for &direction in DIRECTIONS.iter() {
        let mut cursor = Matrix2DNavigator {
          bounds,
          current,
          direction,
        };

        let path = cursor.get_path(4);
        if path.status != PathStatus::Full {
          continue;
        }
        let chars = path
          .points
          .iter()
          .map(|p| input[p.x as usize][p.y as usize]);
        let string: String = chars.collect();
        if string.eq(TARGET) {
          score += 1;
        }
      }
    }
  }
  score
}

fn extra(input: Input) -> Output2 {
  let mut score: Output1 = 0;
  let bounds = Bounds::from_dims(input.len(), input[0].len());

  let check_substring = |s: &str| s.eq("MAS") || s.eq("SAM");
  for (i, line) in input.iter().enumerate().skip(1) {
    if i == (input.len() - 1) {
      break;
    }
    for (j, &c) in line.iter().enumerate().skip(1) {
      if j == (line.len() - 1) {
        break;
      }
      if c != 'A' {
        continue;
      }

      let current = Point::new(i as i64, j as i64);

      //get first diagonal chars
      let direction = Point::new(1, 1);
      let mut cursor = Matrix2DNavigator {
        bounds,
        current: current + direction.opposite(),
        direction,
      };
      let path = cursor.get_path(3);
      let chars = path
        .points
        .iter()
        .map(|p| input[p.x as usize][p.y as usize]);
      let string: String = chars.collect();
      if !check_substring(&string) {
        continue;
      }

      //same for second diagonal
      let direction = Point::new(-1, 1);
      let mut cursor = Matrix2DNavigator {
        bounds,
        current: current + direction.opposite(),
        direction,
      };
      let path = cursor.get_path(3);
      let chars = path
        .points
        .iter()
        .map(|p| input[p.x as usize][p.y as usize]);
      let string: String = chars.collect();
      if !check_substring(&string) {
        continue;
      }

      score += 1;
    }
  }
  score
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
    assert_eq!(score, 18)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 9)
  }
}
