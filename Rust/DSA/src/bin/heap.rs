use std::{fmt, ops::Index};

pub enum HeapType {
    MIN,
    MAX,
}

pub struct Heap<T> {
    heap_type: HeapType,
    data: Vec<T>,
    on_swap: Option<Box<dyn FnMut(usize, usize)>>,
}

impl<T> Index<usize> for Heap<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<T: Ord> Heap<T> {
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            heap_type,
            data: vec![],
            on_swap: None,
        }
    }

    pub fn new_with_handler(heap_type: HeapType, swap_handler: impl FnMut(usize, usize) + 'static) -> Self {
        Self {
            heap_type,
            data: vec![],
            on_swap: Some(Box::new(swap_handler)),
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
                self.swap(0, len - 1);
                let value = self.data.pop();
                self.heapify_down(0);
                value
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn update(&mut self, i: usize, value: T) -> Result<(), &str> {
        if i >= self.data.len() {
            return Err("Invalid item position");
        }
        self.data[i] = value;
        if i != 0 && self.compare(i, self.parent(i).unwrap()) {
            self.heapify_up(i);
        } else {
            self.heapify_down(i);
        }
        Ok(())
    }

    pub fn find<F>(&self, fun: F) -> Option<usize> where F: Fn(&T) -> bool {
        self.data.iter().position(fun)
    }

    pub fn heapify_up(&mut self, i: usize) {
        match self.parent(i) {
            Some(parent) => {
                if !self.compare(parent, i) {
                    self.swap(parent, i);
                    self.heapify_up(parent);
                }
            }
            None => {}
        }
    }

    pub fn heapify_down(&mut self, i: usize) {
        match self.children(i) {
            (Some(left), Some(right)) => {
                let (left_valid, right_valid) = (self.compare(i, left), self.compare(i, right));
                if !left_valid && !right_valid {
                    if self.compare(left, right) {
                        self.swap(i, left);
                        self.heapify_down(left);
                    } else {
                        self.swap(i, right);
                        self.heapify_down(right);
                    }
                } else if !left_valid {
                    self.swap(i, left);
                    self.heapify_down(left);
                } else if !right_valid {
                    self.swap(i, right);
                    self.heapify_down(right);
                }
            }
            (Some(left), None) => {
                if !self.compare(i, left) {
                    self.swap(i, left);
                    self.heapify_down(left);
                }
            }
            _ => {}
        }
    }

    pub fn parent(&self, i: usize) -> Option<usize> {
        match i {
            0 => None,
            _ => Some((i - 1) / 2),
        }
    }

    pub fn children(&self, i: usize) -> (Option<usize>, Option<usize>) {
        let if_exists = |id| if id < self.data.len() { Some(id) } else { None };
        let left = if_exists(2 * i + 1);
        let right = if_exists(2 * i + 2);
        (left, right)
    }

    /// For `(child, parent)` return true if heap is invalid, otherwise false
    ///
    /// - for MIN heap -> heap(left) < heap(right)
    ///
    /// - for MAX heap -> heap(left) > heap(right)
    pub fn compare(&self, left: usize, right: usize) -> bool {
        match self.heap_type {
            HeapType::MIN => self.data[left] < self.data[right],
            HeapType::MAX => self.data[left] > self.data[right],
        }
    }

    pub fn swap(&mut self, left: usize, right: usize) {
        self.data.swap(left, right);
        self.on_swap
            .as_mut()
            .map(|f| f(left, right));
    }

    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }
}

impl<T: fmt::Debug> fmt::Display for Heap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
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
}
