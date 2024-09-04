#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Bitmap<const N: usize> {
    map: [u8; N]
}

impl<const N: usize> Bitmap<{ N }> {
    pub fn new(map: [u8; N]) -> Self {
        Self { map }
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        assert!(self.map.len() * 8 > idx);
        let index = idx / 8;
        let offset = idx % 8;
        let bit_mask = 1u8 << offset;
        if value {
            self.map[index] |= bit_mask;
        } else {
            self.map[index] &= !bit_mask;
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        assert!(self.map.len() * 8 > idx);
        let index = idx / 8;
        let offset = idx % 8;
        let bit_mask = 1u8 << offset;
        self.map[index] & bit_mask != 0
    }

    pub fn reset(&mut self) {
        self.map.iter_mut().for_each(|b| { *b = 0; })
    }
}
