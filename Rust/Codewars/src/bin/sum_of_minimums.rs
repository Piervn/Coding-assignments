fn main() {
    println!(
        "{}",
        sum_of_minimums([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [9, 10, 11, 12]])
    );
}

fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().map(|x| x.iter().min().unwrap()).sum()
}
