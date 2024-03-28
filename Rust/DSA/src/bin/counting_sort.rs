///
/// Counting sort
/// - sorts an array consisting of numbers from <0, k>
/// - stable
/// - time complexity: O(n + k)
/// 

fn counting_sort(arr: &mut Vec<usize>) {
    let k = *arr.iter().max().unwrap();
    let mut counts = vec![0; k + 1];
    for &e in arr.iter() {
        counts[e] += 1;
    }
    for i in 0..k {
        counts[i + 1] += counts[i];
    }
    let cloned_arr = arr.clone();
    for &e in cloned_arr.iter().rev(){
        counts[e] -= 1;
        arr[counts[e]] = e;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = vec![2, 5, 3, 0, 2, 3, 0, 3];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![0, 0, 2, 2, 3, 3, 3, 5]);
    }

    #[test]
    fn test_counting_sort2() {
        let mut arr = vec![6, 8, 7, 4, 9, 1, 0, 0, 0, 5, 6, 8, 9, 7, 1, 4, 3];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![0, 0, 0, 1, 1, 3, 4, 4, 5, 6, 6, 7, 7, 8, 8, 9, 9]);
    }

}