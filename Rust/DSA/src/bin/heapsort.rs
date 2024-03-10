mod heap;

use heap::*;
use std::fmt;

fn heapsort<T: Ord + Clone>(data: &mut [T]) {
    let mut heap = Heap::from_vec(HeapType::MAX, data.to_vec());
    for i in (0..data.len()).rev() {
        data[i] = heap.pop().unwrap();
    }
}

// We save here one comparision for each heapify_down
fn heapsort_with_hole<T: Ord + Clone + fmt::Debug>(data: &mut [T]) {
    let mut heap = Heap::from_vec(HeapType::MAX, data.to_vec());
    for i in (0..data.len()).rev() {
        let mut current = 0;
        loop {
            match heap.children(current) {
                (Some(left), Some(right)) => {
                    // 1 comparision!
                    if heap.compare(left, right) {
                        heap.swap(current, left);
                        current = left;
                    } else {
                        heap.swap(current, right);
                        current = right;
                    }
                }
                (Some(left), None) => {
                    heap.swap(current, left);
                    current = left;
                }
                _ => break,
            }
        }
        if current != i {
            heap.swap(current, i);
            heap.heapify_up(current);
        }
        data[i] = heap[i].clone();
        heap.truncate(i);
    }
}

fn main() {
    println!("Heapsort here!");
}

#[cfg(test)]
mod heapsort_tests {
    #[test]
    fn heapsort_test() {
        let mut data = vec![5, 3, 7, 1, 4, 2, 6, 8];
        super::heapsort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn heapsort_with_hole_test() {
        let mut data = vec![4, 2, 6, 7, 1, 3, 5, 8];
        super::heapsort_with_hole(&mut data);
        //println!("{:?}", data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
