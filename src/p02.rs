extern crate num;

use num::Integer;

fn main() {
  let result: u64 = solve_algorithm(4000000);

  assert_eq!(4613732, result);
}

fn solve_algorithm(top: u64) -> u64 {
  let mut sum: u64 = 0;
  let mut first: u64 = 1;
  let mut second: u64 = 1;
  let mut done = false;

  while !done {
    let current: u64 = first + second;

    if current > top {
      done = true;
    } else {
      first = second;
      second = current;

      if current.is_even() {
        sum += current;
      }
    }
  }
  sum
}