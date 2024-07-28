use core::arch::asm;

use bitflags::bitflags;

bitflags! {
    pub struct DescriptorType: usize {
        const A = 1;
        const R_W = 1 << 1;
        const C_E = 1 << 2;
        const X = 1 << 3;
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SegmentDescriptor([u32;2]);

impl SegmentDescriptor {
    pub fn empty() -> Self {
        Self([0;2])
    }
}

impl SegmentDescriptor {
    /// create segment descriptor
    /// 
    /// address: base address
    /// size: The memory size of the new descriptor
    /// granularity: 1 for 4KB, 0 for 1 byte
    /// S: 1 for data segment, 0 for system segment
    /// descriptor_privilege_level:
    /// present
    /// available: for user
    /// L: 1 for x86_64, 0 for x86
    /// D_B: 1 for 32, 0 for 16
    pub fn new(
        address: u32, 
        size: u32, 
        granularity: bool, 
        descriptor_type: DescriptorType, 
        S: bool, 
        descriptor_privilege_level: usize, 
        present: bool, 
        available: bool, 
        L: bool, 
        D_B: bool) -> Self {
        let mut data: [u32; 2] = [0; 2];
        
        data[0] |= (address << 16) & 0xffff0000;
        data[1] |= (address >> 16) & 0xff;
        data[1] |= address & 0xff000000;

        data[0] |= size & 0xffff;
        data[1] |= size & 0xf0000;

        data[1] |= ((descriptor_type.bits() & 0xf) as u32) << 8;
        if S {
            data[1] |= 1 << 12;
        }
        data[1] |= (descriptor_privilege_level as u32 & 0b11) << 13;
        if present {
            data[1] |= 1 << 15;
        }
        if available {
            data[1] |= 1 << 20;
        }
        if L {
            data[1] |= 1 << 21;
        }
        if D_B {
            data[1] |= 1 << 22;
        }
        if granularity {
            data[1] |= 1 << 23;
        }
        Self(data)
    }
}

impl SegmentDescriptor {
    pub fn address(&self) -> u32 {
        let mut ret = (self.0[0] >> 16) & 0xffff;
        ret |= (self.0[1] & 0xff) << 16;
        ret |= self.0[1] & 0xff000000;
        ret
    }

    pub fn size(&self) -> u32 {
        let mut ret = self.0[0] & 0xffff;
        ret |= self.0[1] & 0x000f0000;
        ret
    }

    pub fn descriptor_type(&self) -> DescriptorType {
        DescriptorType::from_bits_truncate(((self.0[1] >> 8) & 0xf) as usize)
    }

    pub fn S(&self) -> bool {
        ((self.0[1] >> 12) & 1) == 1
    }

    pub fn descriptor_privilege_level(&self) -> usize {
        ((self.0[1] >> 13) & 0b11) as usize
    }

    pub fn present(&self) -> bool {
        ((self.0[1] >> 15) & 1) == 1
    }

    pub fn available(&self) -> bool {
        ((self.0[1] >> 20) & 1) == 1
    }

    pub fn L(&self) -> bool {
        ((self.0[1] >> 21) & 1) == 1
    }

    pub fn D_B(&self) -> bool {
        ((self.0[1] >> 22) & 1) == 1
    }

    pub fn granularity(&self) -> bool {
        ((self.0[1] >> 23) & 1) == 1
    }
}


pub struct GDTRegister(u64);

impl GDTRegister {
    pub fn new(limit: u16, gdt_address: u32) -> Self {
        let mut v = limit as u64;
        v |= (gdt_address as u64) << 16;
        Self(v)
    }

    pub fn set(&mut self, limit: u16, gdt_address: u32) {
        let mut v = limit as u64;
        v |= (gdt_address as u64) << 16;
        self.0 = v;
    }

    pub fn get_limit(&self) -> u16 {
        (self.0 & 0xffff) as u16
    }

    pub fn gdt_address(&self) -> u32 {
        ((self.0 >> 16) & 0xffffffff) as u32
    }
}

pub struct SegmentSelector(u16);

impl SegmentSelector {
    /// create segment selector
    /// 
    /// index
    /// table_indicator: 1 for gdt, 0 for ldt
    /// requested_privilege_level
    pub fn new(index: u16, table_indicator: bool, requested_privilege_level: usize) -> Self {
        let mut value: u16 = (index << 3) | (requested_privilege_level & 0b11) as u16;
        if table_indicator {
            value |= 0b100
        }
        Self(value)
    }
}

impl SegmentSelector {
    pub fn requested_privilege_level(&self) -> usize {
        (self.0 & 0b11) as usize
    }

    pub fn table_indicator(&self) -> bool {
        ((self.0 >> 2) & 1) == 1
    }

    pub fn index(&self) -> u16 {
        self.0 >> 3
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_descriptor() {
        let address: u32 = 0xfff888;
        let size: u32 = 199;
        let granularity = true;
        let descriptor_type = DescriptorType::all();
        let S = true;
        let descriptor_privilege_level: usize = 2;
        let present = false;
        let available = true;
        let L = false;
        let D_B = true;
        let descriptor = SegmentDescriptor::new(address, size, granularity, descriptor_type, S, descriptor_privilege_level, present, available, L, D_B);
        assert_eq!(address, descriptor.address());
        assert_eq!(size, descriptor.size());
        assert_eq!(granularity, descriptor.granularity());
        assert_eq!(descriptor_type, descriptor.descriptor_type());
        assert_eq!(S, descriptor.S());
        assert_eq!(descriptor_privilege_level, descriptor.descriptor_privilege_level());
        assert_eq!(present, descriptor.present());
        assert_eq!(available, descriptor.available());
        assert_eq!(L, descriptor.L());
        assert_eq!(D_B, descriptor.D_B());
    }

    #[test]
    fn test_code_segment_descriptor() {
        let address: u32 = 0x0;
        let size: u32 = u32::MAX & 0xfffff;
        let granularity = true;
        let descriptor_type = DescriptorType::X;
        let S = true;
        let descriptor_privilege_level: usize = 0;
        let present = true;
        let available = false;
        let L = false;
        let D_B = true;
        let descriptor = SegmentDescriptor::new(address, size, granularity, descriptor_type, S, descriptor_privilege_level, present, available, L, D_B);
        assert_eq!(address, descriptor.address());
        assert_eq!(size, descriptor.size());
        assert_eq!(granularity, descriptor.granularity());
        assert_eq!(descriptor_type, descriptor.descriptor_type());
        assert_eq!(S, descriptor.S());
        assert_eq!(descriptor_privilege_level, descriptor.descriptor_privilege_level());
        assert_eq!(present, descriptor.present());
        assert_eq!(available, descriptor.available());
        assert_eq!(L, descriptor.L());
        assert_eq!(D_B, descriptor.D_B());
        println!("code segment descriptor {:#010X} {:#010X}", descriptor.0[0], descriptor.0[1])
    }

    #[test]
    fn test_data_segment_descriptor() {
        let address: u32 = 0x0;
        let size: u32 = u32::MAX & 0xfffff;
        let granularity = true;
        let descriptor_type = DescriptorType::R_W;
        let S = true;
        let descriptor_privilege_level: usize = 0;
        let present = true;
        let available = false;
        let L = false;
        let D_B = true;
        let descriptor = SegmentDescriptor::new(address, size, granularity, descriptor_type, S, descriptor_privilege_level, present, available, L, D_B);
        assert_eq!(address, descriptor.address());
        assert_eq!(size, descriptor.size());
        assert_eq!(granularity, descriptor.granularity());
        assert_eq!(descriptor_type, descriptor.descriptor_type());
        assert_eq!(S, descriptor.S());
        assert_eq!(descriptor_privilege_level, descriptor.descriptor_privilege_level());
        assert_eq!(present, descriptor.present());
        assert_eq!(available, descriptor.available());
        assert_eq!(L, descriptor.L());
        assert_eq!(D_B, descriptor.D_B());
        println!("data segment descriptor {:#010X} {:#010X}", descriptor.0[0], descriptor.0[1])
    }

    #[test]
    fn test_segment_selector() {
        let index: u16 = 1;
        let table_indicator = false;
        let requested_privilege_level: usize = 3;
        let selector = SegmentSelector::new(index, table_indicator, requested_privilege_level);
        assert_eq!(index, selector.index());
        assert_eq!(table_indicator, selector.table_indicator());
        assert_eq!(requested_privilege_level, selector.requested_privilege_level());
    }
}
