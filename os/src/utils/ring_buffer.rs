
#[repr(C)]
pub struct RingBuffer<T, const N: usize> {
    array: [T; N],
    start_index: usize,
    end_index: usize,
    is_empty: bool,
}

impl<T, const N: usize> RingBuffer<T, { N }> {
    pub fn new(init_value: T) -> Self where T: core::marker::Copy {
        assert!(N > 0);
        Self { array: [init_value; N], start_index: 0, end_index: 0, is_empty: true }
    }

    pub fn is_empty(&self) -> bool {
        self.start_index == self.end_index && self.is_empty
    }

    pub fn is_full(&self) -> bool {
        self.start_index == self.end_index && !self.is_empty
    }

    pub fn push(&mut self, value: T) -> bool {
        if self.is_full() {
            return false;
        }
        self.array[self.start_index % N] = value;
        self.start_index = (self.start_index + 1) % N;
        self.is_empty = false;
        true
    }

    pub fn pop(&mut self) -> Option<T> where T: core::marker::Copy {
        if self.is_empty() {
            return None;
        }

        let value = self.array[self.end_index % N];
        self.end_index = (self.end_index + 1) % N;
        if self.start_index == self.end_index {
            self.is_empty = true;
        }
        Some(value)
    }
}
