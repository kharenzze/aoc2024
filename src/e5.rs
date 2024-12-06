use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = (Vec<String>, Vec<String>);
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 5;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
  let mut line_iter = lines.iter();
  let rules = line_iter
    .take_while_ref(|l| !l.is_empty())
    .map(|l| l.clone())
    .collect();
  let cases = line_iter.skip(1).map(|l| l.clone()).collect();
  (rules, cases)
}

#[derive(Debug)]
struct Rule {
  left: i64,
  right: i64,
}

#[derive(Debug)]
struct Case(Vec<i64>);

impl Case {
  fn parse(s: &str) -> Self {
    Self(s.split(",").map(|n| n.parse().unwrap()).collect())
  }

  fn is_valid(&self, left_wing: &Wing) -> bool {
    let l = self.0.len();
    for (i, left) in self.0.iter().take(l - 1).enumerate() {
      let mut right = self.0.iter().skip(i + 1);
      let wing = left_wing.get(left);
      if wing.is_none() {
        continue;
      }
      let wing = wing.unwrap();
      let invalid = right.any(|r| wing.contains(r));
      if invalid {
        return false;
      }
    }
    true
  }

  fn fix(&self, left_wing: &Wing) -> Self {
    let mut new_case = self.0.clone();
    let l = self.0.len();
    'main: loop {
      for (i, left) in new_case.iter().take(l - 1).enumerate() {
        let wing = left_wing.get(left);
        if wing.is_none() {
          continue;
        }
        let wing = wing.unwrap();

        let invalid = {
          let ref case = new_case;
          let mut right = case.iter().enumerate().skip(i + 1);
          let occurrence = right.find(|(_, r)| wing.contains(r));
          occurrence
        };

        if let Some((j, &value)) = invalid {
          new_case.remove(j);
          new_case.insert(i, value);

          //try again
          continue 'main;
        }
      }
      break;
    }
    Self(new_case)
  }

  fn get_middle_value(&self) -> i64 {
    self.0[self.0.len() / 2]
  }
}

impl Rule {
  fn parse(s: &str) -> Self {
    let mut parts = s.split("|");
    let left = parts.next().unwrap().parse().unwrap();
    let right = parts.next().unwrap().parse().unwrap();
    Self { left, right }
  }
}

type Wing = HashMap<i64, HashSet<i64>>;

fn initial(input: Input) -> Output1 {
  let mut left_wing: Wing = HashMap::new();
  let mut right_wing: Wing = HashMap::new();
  let rules_iter = input.0.iter().map(|r| Rule::parse(r));
  for rule in rules_iter {
    left_wing.entry(rule.right).or_default().insert(rule.left);
    right_wing.entry(rule.left).or_default().insert(rule.right);
  }

  let cases_iter = input.1.iter().map(|c| Case::parse(c));
  let invalid = cases_iter.filter(|c| c.is_valid(&left_wing));
  let score: Output1 = invalid.map(|c| c.get_middle_value()).sum();
  score
}

fn extra(input: Input) -> Output2 {
  let mut left_wing: Wing = HashMap::new();
  let mut right_wing: Wing = HashMap::new();
  let rules_iter = input.0.iter().map(|r| Rule::parse(r));
  for rule in rules_iter {
    left_wing.entry(rule.right).or_default().insert(rule.left);
    right_wing.entry(rule.left).or_default().insert(rule.right);
  }

  let cases_iter = input.1.iter().map(|c| Case::parse(c));
  let invalid = cases_iter
    .filter(|c| !c.is_valid(&left_wing))
    .map(|c| c.fix(&left_wing));
  let score: Output2 = invalid.map(|c| c.get_middle_value()).sum();
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
  fn read() {
    let input = read_data(true);
    assert_ne!(input.0.len(), 0);
    assert_ne!(input.1.len(), 0);
  }

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 143)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 123)
  }
}
