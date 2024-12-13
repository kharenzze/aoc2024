use std::fs::File;
use std::io::{prelude::*, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

type Input = Vec<String>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 7;

#[derive(Debug, Clone)]
struct Request {
  result: i64,
  values: Vec<i64>,
}

lazy_static! {
  static ref REQUEST_REGEX: Regex = Regex::new(r"(\d+): (.*)$").unwrap();
}

impl Request {
  fn from(line: &str) -> Self {
    let captures = REQUEST_REGEX.captures(line).unwrap();
    let result: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
    let values = captures
      .get(2)
      .unwrap()
      .as_str()
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();
    Self { result, values }
  }

  fn is_valid(&self) -> bool {
    let l = self.values.len();
    let l_ops = l - 1;
    let mut operations: Vec<Operation> = vec![];
    let mut accs = vec![self.values[0]];

    loop {
      let mut depth = operations.len();
      if depth < l_ops {
        //go deeper
        let op = OperationCursor::first();
        operations.push(op);
        let last_acc = accs.last().unwrap();
        let next_value = self.values[depth + 1];
        let next_acc = op.apply(*last_acc, next_value);
        accs.push(next_acc);
        continue;
      }

      let last_acc = accs.last().unwrap();
      if last_acc == &self.result {
        return true;
      }

      //go up
      loop {
        let op = operations.pop().unwrap();
        accs.pop();
        depth -= 1;
        let Some(next_op) = OperationCursor::next(&op) else {
          if depth == 0 {
            return false;
          }
          continue;
        };
        operations.push(next_op);
        let next_value = self.values[depth + 1];
        let last_acc = accs.last().unwrap();
        let next_acc = next_op.apply(*last_acc, next_value);
        accs.push(next_acc);
        break;
      }
    }
  }

  fn is_valid_v2(&self) -> bool {
    let l = self.values.len();
    let l_ops = l - 1;
    let mut operations: Vec<Operation> = vec![];
    let mut accs = vec![self.values[0]];

    loop {
      let mut depth = operations.len();
      if depth < l_ops {
        //go deeper
        let op = OperationCursorV2::first();
        operations.push(op);
        let last_acc = accs.last().unwrap();
        let next_value = self.values[depth + 1];
        let next_acc = op.apply(*last_acc, next_value);
        accs.push(next_acc);
        continue;
      }

      let last_acc = accs.last().unwrap();
      if last_acc == &self.result {
        return true;
      }

      //go up
      loop {
        let op = operations.pop().unwrap();
        accs.pop();
        depth -= 1;
        let Some(next_op) = OperationCursorV2::next(&op) else {
          if depth == 0 {
            return false;
          }
          continue;
        };
        operations.push(next_op);
        let next_value = self.values[depth + 1];
        let last_acc = accs.last().unwrap();
        let next_acc = next_op.apply(*last_acc, next_value);
        accs.push(next_acc);
        break;
      }
    }
  }
}

struct OperationCursor {}

impl OperationCursor {
  const fn next(op: &Operation) -> Option<Operation> {
    match op {
      Operation::Add => Some(Operation::Mul),
      Operation::Mul => None,
      _ => unreachable!(),
    }
  }

  const fn first() -> Operation {
    Operation::Add
  }
}

struct OperationCursorV2 {}

impl OperationCursorV2 {
  const fn next(op: &Operation) -> Option<Operation> {
    match op {
      Operation::Add => Some(Operation::Mul),
      Operation::Mul => Some(Operation::Concat),
      Operation::Concat => None,
    }
  }

  const fn first() -> Operation {
    Operation::Add
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
  Add,
  Mul,
  Concat,
}

impl Operation {
  fn apply(&self, a: i64, b: i64) -> i64 {
    match self {
      Operation::Add => a + b,
      Operation::Mul => a * b,
      Operation::Concat => {
        let a_str = a.to_string();
        let b_str = b.to_string();
        format!("{}{}", a_str, b_str).parse().unwrap()
      }
    }
  }
}

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn initial(input: Input) -> Output1 {
  input
    .into_iter()
    .map(|l| Request::from(&l))
    .filter(Request::is_valid)
    .map(|r| r.result)
    .sum()
}

fn extra(input: Input) -> Output2 {
  input
    .into_iter()
    .map(|l| Request::from(&l))
    .filter(Request::is_valid_v2)
    .map(|r| r.result)
    .sum()
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
    assert_eq!(score, 3749)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 11387)
  }
}
