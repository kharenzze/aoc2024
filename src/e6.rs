use crate::point::{Bounds, Point};
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 6;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug)]
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

  fn rotate(&self) -> Self {
    match self {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
    }
  }
}

#[derive(Debug)]
enum Cell {
  Empty,
  Wall,
  Guard(Direction),
}

impl Cell {
  const fn from(c: char) -> Self {
    match c {
      '.' => Cell::Empty,
      '#' => Cell::Wall,
      _ => Cell::Guard(Direction::Up),
    }
  }
}

struct GameCursor {
  position: Point,
  bounds: Bounds,
  direction: Direction,
}

impl GameCursor {
  fn next(&self) -> Option<Point> {
    let next = self.position + self.direction.to_point();
    if next.is_in_bounds(&self.bounds) {
      Some(next)
    } else {
      None
    }
  }

  fn advance(&mut self) -> Option<Point> {
    self.next().map(|p| {
      self.position = p;
      p
    })
  }

  fn rotate(&mut self) {
    self.direction = self.direction.rotate();
  }
}

#[derive(Debug)]
struct Game {
  map: Vec<Vec<Cell>>,
}

impl Game {
  fn new(input: Input) -> Self {
    let map = input
      .iter()
      .map(|l| l.chars().map(|c| Cell::from(c)).collect())
      .collect();
    Self { map }
  }

  fn guard_position(&self) -> Point {
    for (i, line) in self.map.iter().enumerate() {
      for (j, cell) in line.iter().enumerate() {
        if let Cell::Guard(_) = cell {
          return Point::new(j as i64, i as i64);
        }
      }
    }
    unreachable!()
  }
}

fn initial(input: Input) -> Output1 {
  let mut visited: HashSet<Point> = HashSet::new();
  let game = Game::new(input);
  let mut cursor = GameCursor {
    position: game.guard_position(),
    bounds: Bounds::from_dims(game.map.len(), game.map[0].len()),
    direction: Direction::Up,
  };

  loop {
    visited.insert(cursor.position);
    let next = cursor.next();
    if next.is_none() {
      //out of bounds
      break;
    }
    let next = next.unwrap();
    let ref next_cell = game.map[next.y as usize][next.x as usize];
    if let Cell::Wall = next_cell {
      cursor.rotate();
    } else {
      cursor.advance();
    }
  }

  visited.len() as i64
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
    assert_eq!(score, 41)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
