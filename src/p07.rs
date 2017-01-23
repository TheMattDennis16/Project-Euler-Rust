//Could implement as Sieve of Eratosthenes
extern crate primal;

fn solve(top: u64) -> u64 {
    primal::Primes::all().nth(top as usize).unwrap() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(104743, super::solve_lib(10001 - 1));
    }
}