use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();  
    input.clear();
    stdin().read_line(&mut input).unwrap();  
    let mut arr = input.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    arr.sort();
    arr[1..].chunks_exact_mut(2).for_each(|x| x.swap(0, 1));

    println!("{}", arr.iter().map(ToString::to_string).collect::<Vec<String>>().join(" "));
}