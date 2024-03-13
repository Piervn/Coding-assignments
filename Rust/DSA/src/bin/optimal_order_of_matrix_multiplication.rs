///
/// Optimal order of matrix multiplication
/// 
/// Optimal cost:
///     Input:
///         dimensions: d_0, d_1, d_2, ..., d_n
///     Output:
///         Optimal cost of multiplication matrices with following dimensions:
///         - M1: d_0 x d_1
///         - M2: d_1 x d_2
///         - ...
///         - Mn: d_{n-1} x d_n
///

fn calculate_cost(dimensions: Vec<u32>) -> u32 {
    let dp_size = dimensions.len() - 1;
    let mut dp = vec![vec![0; dp_size]; dp_size];

    for d in 0..dp_size {
        dp[d][d] = 0;
    }

    for d in 1..dp_size {
        
    }
    return 0;
}

fn main() {
    let dimensions = vec![1, 2, 3, 4, 5];
    println!("{}", calculate_cost(dimensions));
}
    
    
    
    
    