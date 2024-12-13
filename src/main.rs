mod e1;
mod e2;
mod e3;
mod e4;
mod e5;
mod e6;
mod e7;
mod e8;
mod point;

use std::env;
use std::time::Instant;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("Invalid number of arguments")
  }

  let day: &str = args.get(1).unwrap();
  let part: usize = args.get(2).unwrap().parse().unwrap();

  let now = Instant::now();

  match day {
    "1" => crate::e1::solve(part),
    "2" => crate::e2::solve(part),
    "3" => crate::e3::solve(part),
    "4" => crate::e4::solve(part),
    "5" => crate::e5::solve(part),
    "6" => crate::e6::solve(part),
    "7" => crate::e7::solve(part),
    "8" => crate::e8::solve(part),
    _ => unreachable!(),
  }

  let elapsed = now.elapsed();
  println!("Elapsed: {elapsed:?}");
}
