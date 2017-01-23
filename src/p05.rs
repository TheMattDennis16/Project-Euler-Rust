extern crate primes;

fn main() {
  assert_eq!(232792560, solve(1, 20));
}

fn solve(bottom: u64, top: u64) -> u64 {
  let mut answer: u64 = 0;
  let mut i: u64 = 0;

  loop {
    i += 20;
    if passed_division(i, bottom, top) {
      answer = i;
      break;
    }
  }
  answer
}

fn passed_division(num: u64, bottom: u64, top: u64) -> bool {
  let mut passed = true;
  for i in bottom..top {
    if num % i != 0 {
      passed = false;
      break;
    }
  }
  passed
}

#[cfg(test)]
mod tests {
  #[test]
  fn solve_small() {
    assert_eq!(2520, super::solve(1, 10));
  }

  #[test]
  fn solve_large() {
    assert_eq!(232792560, super::solve(1, 20));
  }
}