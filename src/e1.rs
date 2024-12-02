use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = (Vec<i64>, Vec<i64>, usize);

const DAY: usize = 1;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  let input_lines: Vec<String> = line_iter.map(|l| l.unwrap()).collect();
  let capacity = input_lines.len();
  let mut v1: Vec<i64> = vec![0; capacity];
  let mut v2: Vec<i64> = vec![0; capacity];
  for (i, line) in input_lines.iter().enumerate() {
    let mut cursor = line.split_whitespace();
    v1[i] = cursor.next().unwrap().parse().unwrap();
    v2[i] = cursor.next().unwrap().parse().unwrap();
  }
  (v1, v2, capacity)
}

fn initial(input: Input) -> i64 {
  let (v1, v2, capacity) = input;
  let v1_sorted: Vec<i64> = v1.iter().sorted().cloned().collect();
  let v2_sorted: Vec<i64> = v2.iter().sorted().cloned().collect();
  let mut difs: Vec<i64> = vec![0; capacity];
  for i in 0..capacity {
    difs[i] = (v1_sorted[i] - v2_sorted[i]).abs();
  }
  let sum: i64 = difs.iter().sum();
  sum
}

fn extra(input: Input) -> usize {
  unimplemented!()
}

pub fn solve() {
  let input = read_data(false);
  let score = initial(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 11)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
