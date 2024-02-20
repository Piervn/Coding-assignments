enum HeapType {
    MIN,
    MAX,
}

struct Heap<T> {
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

    pub fn heapify_up(&mut self, i: usize) {
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

    pub fn heapify_down(&mut self, i: usize) {
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
}

fn main() {
    let mut heap = Heap::new(HeapType::MAX);
    heap.push(5);
    heap.push(3);
    heap.push(7);
    heap.push(1);
    heap.push(4);
    heap.push(2);
    heap.push(6);
    heap.push(8);
    println!("{:?}", heap.data);
    println!("{:?}", heap.pop());
    println!("{:?}", heap.data);
}
