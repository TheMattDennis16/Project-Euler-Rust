fn solve_algorithm(top: u64) -> u64 {
  let mut to_return: u64 = 0;
  'out: for a in 2..top {
    for b in 2..a {
      if sq(a) + sq(b) == sq(top - a - b) {
        to_return = a * b * (top-a-b);
        break 'out;
      }
    }
  }
  to_return
}

fn sq(num: u64) -> u64 {
  num * num
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_solve() {
    assert_eq!(31875000, super::solve_algorithm(1000));
  }

  #[test]
  fn test_square() {
    assert_eq!(100, super::sq(10));
  }
}