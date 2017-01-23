fn main() {
    assert_eq!(25164150, solve(100));
}

fn solve(top: u64) -> u64 {
    get_square_of_sum(top) - get_sum_of_squares(top)
}

fn get_sum_of_squares(top: u64) -> u64 {
    top * (top + 1) * (2 * top + 1) / 6
    //(0..top + 1).fold(0, |sum, x| sum + sq(x))
}

fn get_square_of_sum(top: u64) -> u64 {
    sq(top * (top + 1)/2)
    //sq((0..top + 1).fold(0, |sum, x| sum + x))
}

fn sq(top: u64) -> u64 {
    top*top
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_of_square() {
        assert_eq!(385, super::get_sum_of_squares(10));
    }

    #[test]
    fn test_square_of_sum() {
        assert_eq!(3025, super::get_square_of_sum(10));
    }

    #[test]
    fn test_solve() {
        assert_eq!(25164150, super::solve(100));
    }
}