use alloc::sync::Arc;
use spin::Mutex;

use super::Bitmap;

pub struct IdAllocator<const N: usize> {
    bitmap: Bitmap<N>,
    base: usize,
    begin: usize,
    end: usize,
    current: usize,
}

impl<const N: usize> IdAllocator<{ N }> {
    pub fn new(bitmap: Bitmap<N>, base: usize, begin: usize, end: usize, current: usize) -> Self {
        assert!(begin < end);
        assert!(begin <= current);
        assert!(current <= end);
        assert!((end - begin) <= N * 8);
        Self { bitmap, base, begin, end, current}
    }

    pub fn alloc(&mut self) -> Option<usize> {
        for idx in self.begin..self.current {
            if !self.get_bitmap(idx) {
                self.set_bitmap(idx, true);
                return Some(idx);
            }
        }
        if self.current < self.end {
            self.set_bitmap(self.current, true);
            self.current += 1;
            return Some(self.current - 1);
        }
        None
    }

    pub fn dealloc(&mut self, idx: usize) {
        let old_value = self.get_bitmap(idx);
        assert!(old_value, "Id in #{} is not used", idx);
        self.set_bitmap(idx, false);
        let mut idx = idx;
        if idx == self.current - 1 {
            while !self.get_bitmap(idx) {
                self.current = idx;
                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }
    }

    fn inner_index(&self, outer_index: usize) -> usize {
        outer_index - self.base
    }

    fn set_bitmap(&mut self, idx: usize, value: bool) {
        self.bitmap.set(self.inner_index(idx), value)
    }

    fn get_bitmap(&self, idx: usize) -> bool {
        self.bitmap.get(self.inner_index(idx))
    }
}


pub struct IdAllocatorWithReferenceBitmap<'a, const N: usize> {
    bitmap: &'a mut Bitmap<N>,
    base: usize,
    begin: usize,
    end: usize,
    current: usize,
}

impl<'a, const N: usize> IdAllocatorWithReferenceBitmap<'a, { N }> {
    pub fn new(bitmap: &'a mut Bitmap<N>, base: usize, begin: usize, end: usize, current: usize) -> Self {
        assert!(begin < end);
        assert!(begin <= current);
        assert!(current <= end);
        assert!((end - begin) <= N * 8);
        Self { bitmap, base, begin, end, current}
    }

    pub fn alloc(&mut self) -> Option<usize> {
        for idx in self.begin..self.current {
            if !self.get_bitmap(idx) {
                self.set_bitmap(idx, true);
                return Some(idx);
            }
        }
        if self.current < self.end {
            self.set_bitmap(self.current, true);
            self.current += 1;
            return Some(self.current - 1);
        }
        None
    }

    pub fn dealloc(&mut self, idx: usize) {
        let old_value = self.get_bitmap(idx);
        assert!(old_value, "Id in #{} is not used", idx);
        self.set_bitmap(idx, false);
        let mut idx = idx;
        if idx == self.current - 1 {
            while !self.get_bitmap(idx) {
                self.current = idx;
                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }
    }

    fn inner_index(&self, outer_index: usize) -> usize {
        outer_index - self.base
    }

    fn set_bitmap(&mut self, idx: usize, value: bool) {
        self.bitmap.set(self.inner_index(idx), value)
    }

    fn get_bitmap(&self, idx: usize) -> bool {
        self.bitmap.get(self.inner_index(idx))
    }
}


pub struct IdStub<const N: usize> {
    pub id: usize,
    pub id_allocator: Arc<Mutex<IdAllocator<N>>>,
}

impl<const N: usize> IdStub<{ N }> {
    pub fn new(id: usize, id_allocator: Arc<Mutex<IdAllocator<N>>>) -> Self {
        Self { id, id_allocator: id_allocator.clone() }
    }
}

impl<const N: usize> Drop for IdStub<{ N }> {
    fn drop(&mut self) {
        self.id_allocator.lock().dealloc(self.id);
    }
}
