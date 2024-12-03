use regex::Regex;

type Input = String;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 3;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let content =
    std::fs::read_to_string(&filename).expect(&format!("Cannot open file {}", &filename));
  content
}

fn initial(input: Input) -> Output1 {
  let re = Regex::new(r"mul\(((\d{1,3})),(\d{1,3})\)").unwrap();
  let mut score = 0;
  for cap in re.captures_iter(&input) {
    let a = cap[2].parse::<i64>().unwrap();
    let b = cap[3].parse::<i64>().unwrap();
    score += a * b;
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
    assert_eq!(score, 161)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 13)
  }
}
