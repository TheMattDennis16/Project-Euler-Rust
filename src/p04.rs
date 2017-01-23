fn main() {
  assert_eq!(906609, solve(100,999));
}

fn solve(bottom: u64, top: u64) -> u64 {
  let mut result: u64 = 0;
  for i in bottom..(top+1) {
    for y in bottom..(top+1) {
      let number: u64 = i * y;
      if is_palindrome(number) && number > result {
        result = number;
      }
    }
  }
  result
}

fn is_palindrome(number: u64) -> bool {
  let back_string: String = number.to_string().chars().rev().collect();
  number.to_string() == back_string
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_small() {
    assert_eq!(9009, super::solve(1, 99));
  }

  #[test]
  fn test_large() {
    assert_eq!(906609, super::solve(100, 999));
  }
}