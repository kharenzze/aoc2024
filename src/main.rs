mod e1;
mod e2;
mod e3;
mod e4;
mod e5;
mod point;

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
    "2" => crate::e2::solve(part),
    "3" => crate::e3::solve(part),
    "4" => crate::e4::solve(part),
    "5" => crate::e5::solve(part),
    _ => unreachable!(),
  }
}
