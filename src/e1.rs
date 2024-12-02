use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = (Vec<i64>, Vec<i64>, usize);
type Output1 = i64;
type Output2 = i64;

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

fn initial(input: Input) -> Output1 {
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

fn extra(input: Input) -> Output2 {
  let (v1, v2, capacity) = input;
  let mut hist: HashMap<i64, i64> = HashMap::new();
  for i in 0..capacity {
    let key = v2[i];
    *hist.entry(key).or_insert(0) += 1;
  }

  let values: Vec<i64> = v1
    .iter()
    .map(|v| {
      let instances = hist.get(v).or(Some(&0)).cloned().unwrap();
      instances * v
    })
    .collect();
  let sum: i64 = values.iter().sum();
  sum
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
    assert_eq!(score, 11)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 31)
  }
}
