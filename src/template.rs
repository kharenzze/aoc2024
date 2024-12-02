use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 1;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn initial(input: Input) -> Output1 {
  unimplemented!()
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
    assert_eq!(score, 13)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
