extern crate primes;

fn main() {
  assert_eq!(6857, solve(600851475143));
}

fn solve(top: u64) -> u64 {
  let set = *(primes::PrimeSet::new()
    .prime_factors(top)
    .iter()
    .max())
    .unwrap();
  set
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_solve() {
    assert_eq!(6857, super::solve(600851475143));
  }
}