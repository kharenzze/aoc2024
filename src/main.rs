mod e1;

use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("Invalid number of arguments")
  }

  let day: &str = args.get(1).unwrap();
  let part: usize = args.get(2).unwrap().parse().unwrap();

  match day {
    "1" => crate::e1::solve(part),
    _ => unreachable!(),
  }
}
