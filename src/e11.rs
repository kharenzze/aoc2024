use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<i64>;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 11;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line = reader.lines().next().unwrap().unwrap();
  line
    .split_whitespace()
    .map(|v| v.parse().unwrap())
    .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Task {
  depth: i64,
  value: i64,
}

impl Ord for Task {
  fn cmp(&self, other: &Self) -> Ordering {
    let order = other.depth.cmp(&self.depth);
    if order == Ordering::Equal {
      other.value.cmp(&self.value)
    } else {
      order
    }
  }
}

impl PartialOrd for Task {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Task {
  fn try_resolve(&self) -> Option<Vec<i64>> {
    if self.depth != 1 {
      return None;
    }
    if self.value == 0 {
      return Some(vec![1]);
    }
    let text = self.value.to_string();
    if text.len() % 2 == 0 {
      //split in two
      let (left, right) = text.split_at(text.len() / 2);
      let left_value = left.parse::<i64>().unwrap();
      let right_value = right.parse::<i64>().unwrap();
      return Some(vec![left_value, right_value]);
    }

    Some(vec![self.value * 2024])
  }
}

fn solve_initial(input: Input, depth: i64) -> Output1 {
  let mut pending_explore: BinaryHeap<Task> =
    input.iter().map(|v| Task { depth, value: *v }).collect();

  let mut cache: HashMap<Task, i64> = HashMap::new();

  while let Some(task) = pending_explore.pop() {
    //try read cache
    if cache.get(&task).is_some() {
      continue;
    };

    //try solve
    if let Some(result) = task.try_resolve() {
      cache.insert(task, result.len() as i64);
      continue; //solved!
    };

    //go deeper
    let children_task = Task {
      depth: 1,
      value: task.value,
    };

    let Some(children) = children_task.try_resolve() else {
      unreachable!();
    };

    let children_values: Vec<Option<&i64>> = children
      .iter()
      .map(|v| Task {
        depth: task.depth - 1,
        value: *v,
      })
      .map(|t| {
        let Some(v) = cache.get(&t) else {
          pending_explore.push(t);
          return None;
        };
        Some(v)
      })
      .collect();

    if children_values.iter().any(|v| v.is_none()) {
      pending_explore.push(task);
      continue;
    }

    let result: i64 = children_values.into_iter().filter_map(|v| v).cloned().sum();

    cache.insert(task, result);
  }

  input
    .iter()
    .map(|v| *cache.get(&Task { depth, value: *v }).unwrap())
    .sum()
}

fn initial(input: Input) -> Output1 {
  solve_initial(input, 25)
}

fn extra(input: Input) -> Output2 {
  solve_initial(input, 75)
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
  fn sorting() {
    let mut heap: BinaryHeap<Task> = BinaryHeap::new();
    heap.push(Task { depth: 2, value: 2 });
    heap.push(Task { depth: 1, value: 1 });
    heap.push(Task { depth: 3, value: 3 });
    assert_eq!(heap.pop().unwrap().value, 1);
    assert_eq!(heap.pop().unwrap().value, 2);
    assert_eq!(heap.pop().unwrap().value, 3);
  }

  #[test]
  fn manual() {
    let score = solve_initial(vec![125], 2);
    assert_eq!(score, 2)
  }

  #[test]
  fn manual2() {
    let score = solve_initial(vec![0], 10);
    println!("{:?}", score);
  }

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = solve_initial(input.clone(), 6);
    assert_eq!(score, 22);
    let score = initial(input);
    assert_eq!(score, 55312)
  }
}
