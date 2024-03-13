///
/// Optimal order of matrix multiplication
/// - time complexity: O(n^3)
/// - assumption: multiplication of 2 matrices a x b and b x c costs a * b * c
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

fn calculate_cost(dims: Vec<u32>) -> u32 {
    let dp_size = dims.len() - 1;
    let mut dp = vec![vec![0; dp_size]; dp_size];

    for diag in 0..dp_size {
        for i in 0..(dp_size - diag) {
            let j = i + diag;
            let cost = {i..j}.map(
                |k| 
                dp[i][k] + dp[k + 1][j] + 
                dims[i] * dims[k + 1] * dims[j + 1]
            ).min().unwrap_or(0);
            dp[i][j] = cost;
        }
    }

    dp[0][dp_size - 1]
}

fn main() {
    let dimensions = vec![2, 3, 8, 1, 4];
    println!("{}", calculate_cost(dimensions));
}
    