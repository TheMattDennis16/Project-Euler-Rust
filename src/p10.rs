extern crate primal;

fn solve(top: u64) -> u64 {
  primal::Primes::all().take_while(|p| *p < top as usize).fold(0, |sum, i| sum + i as u64)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_solve() {
    assert_eq!(142913828922, super::solve(2000000));
  }
}