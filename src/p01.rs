fn main() {
    let sum: i32 = get_sum(1000);
    println!("The answer is: {}", sum);
}

fn get_sum(top: i32) -> i32 {
    let mut sum = 0;
    for val in 1..top {
        if val % 3 == 0 || val % 5 == 0 {
            sum += val;
        }
    }
    sum
}