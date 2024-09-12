use std::io::stdin;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { return a };
    return gcd(b, a % b);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();  
    let n = input.trim().parse::<i32>().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();  
    let arr = input.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let result = arr.iter().fold(0, |acc, x| gcd(acc, *x));
    println!("{}", result * n);
}