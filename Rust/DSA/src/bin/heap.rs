use std::fmt;

pub enum HeapType {
    MIN,
    MAX,
}

pub struct Heap<T> {
    heap_type: HeapType,
    data: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            heap_type,
            data: vec![],
        }
    }

    pub fn from_vec(heap_type: HeapType, data: Vec<T>) -> Self {
        let mut heap = Self::new(heap_type);
        heap.data = data;
        for i in (0..heap.data.len()).rev() {
            heap.heapify_down(i);
        }
        heap
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.data.len();
        match len {
            0 => None,
            _ => {
                self.data.swap(0, len - 1);
                let value = self.data.pop();
                self.heapify_down(0);
                value
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    fn heapify_up(&mut self, i: usize) {
        match self.parent(i) {
            Some(parent) => {
                if !self.compare(parent, i) {
                    self.data.swap(parent, i);
                    self.heapify_up(parent);
                }
            }
            None => {}
        }
    }

    fn heapify_down(&mut self, i: usize) {
        match self.children(i) {
            (Some(left), Some(right)) => {
                let (left_valid, right_valid) = (self.compare(i, left), self.compare(i, right));
                if !left_valid && !right_valid {
                    if self.compare(left, right) {
                        self.data.swap(i, left);
                        self.heapify_down(left);
                    } else {
                        self.data.swap(i, right);
                        self.heapify_down(right);
                    }
                } else if !left_valid {
                    self.data.swap(i, left);
                    self.heapify_down(left);
                } else if !right_valid {
                    self.data.swap(i, right);
                    self.heapify_down(right);
                }
            }
            (Some(left), None) => {
                if !self.compare(i, left) {
                    self.data.swap(i, left);
                    self.heapify_down(left);
                }
            }
            _ => {}
        }
    }

    fn parent(&self, i: usize) -> Option<usize> {
        match i {
            0 => None,
            _ => Some((i - 1) / 2),
        }
    }

    fn children(&self, i: usize) -> (Option<usize>, Option<usize>) {
        let if_exists = |id| if id < self.data.len() { Some(id) } else { None };
        let left = if_exists(2 * i + 1);
        let right = if_exists(2 * i + 2);
        (left, right)
    }

    /// for MIN heap -> heap(left) < heap(right)
    ///
    /// for MAX heap -> heap(left) > heap(right)
    fn compare(&self, left: usize, right: usize) -> bool {
        match self.heap_type {
            HeapType::MIN => self.data[left] < self.data[right],
            HeapType::MAX => self.data[left] > self.data[right],
        }
    }

    fn swap(&mut self, left: usize, right: usize) {
        self.data.swap(left, right);
    }
}

impl<T: fmt::Debug> fmt::Display for Heap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

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
        data[i] = heap.data[i].clone();
        heap.data.truncate(i);
    }
}

fn main() {
    let heap = Heap::from_vec(HeapType::MAX, vec![5, 3, 7, 1, 4, 2, 6, 8]);
    println!("{}", heap);
}

#[cfg(test)]
mod heap_tests {
    #[test]
    fn min_heap_test() {
        let mut heap = super::Heap::new(super::HeapType::MIN);
        heap.push(5);
        heap.push(3);
        heap.push(7);
        heap.push(1);
        heap.push(4);
        heap.push(2);
        heap.push(6);
        heap.push(8);
        while heap.data.len() > 0 {
            let root = heap.peek().unwrap();
            assert!(root <= heap.data.iter().max().unwrap());
            heap.pop().unwrap();
        }
    }

    #[test]
    fn max_heap_test() {
        let mut heap = super::Heap::new(super::HeapType::MAX);
        heap.push(5);
        heap.push(3);
        heap.push(7);
        heap.push(1);
        heap.push(4);
        heap.push(2);
        heap.push(6);
        heap.push(8);
        while heap.data.len() > 0 {
            let root = heap.peek().unwrap();
            assert!(root >= heap.data.iter().min().unwrap());
            heap.pop().unwrap();
        }
    }

    #[test]
    fn heap_from_vec_test() {
        let data = vec![5, 3, 7, 1, 4, 2, 6, 8];
        let mut heap_from_vec = super::Heap::from_vec(super::HeapType::MAX, data);

        while heap_from_vec.data.len() > 0 {
            let root = heap_from_vec.peek().unwrap();
            assert!(root >= heap_from_vec.data.iter().min().unwrap());
            heap_from_vec.pop().unwrap();
        }
    }

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