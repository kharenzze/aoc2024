use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::point::{Bounds, Point};

type Input = Vec<String>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 8;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn initial(input: Input) -> Output1 {
  let game = Game::new(input);
  game.solve()
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

#[derive(Debug, Clone)]
struct Game {
  grid: Vec<Vec<char>>,
  pos_map: HashMap<char, HashSet<Point>>,
  bounds: Bounds,
}

impl Game {
  fn new(input: Input) -> Self {
    let grid: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
    let mut pos_map: HashMap<char, HashSet<Point>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
      for (x, &c) in row.iter().enumerate() {
        if c != '.' {
          let point = Point::new(x as i64, y as i64);
          pos_map.entry(c).or_default().insert(point);
        }
      }
    }
    let y = grid.len();
    let x = grid[0].len();
    let bounds = Bounds::from_dims(x, y);

    Self {
      grid,
      pos_map,
      bounds,
    }
  }

  fn solve(&self) -> i64 {
    let mut antenna_pos: HashSet<Point> = HashSet::new();
    for (_, points) in self.pos_map.iter() {
      let n = points.len();
      if n < 2 {
        continue;
      }
      for (i, &a) in points.iter().enumerate().take(n - 1) {
        for &b in points.iter().skip(i + 1) {
          let diff = (b - a);
          let antenna = a - diff;
          if self.bounds.check(&antenna) {
            antenna_pos.insert(antenna);
          }
          let antenna = b + diff;
          if self.bounds.check(&antenna) {
            antenna_pos.insert(antenna);
          }
        }
      }
    }
    antenna_pos.len() as i64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 14)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
