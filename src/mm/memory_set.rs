use core::convert::From;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use bitflags::bitflags;

use crate::arch::x86::PteFlags;

use super::{alloc_frame, page_table::{self, PageTable}, FrameTracker, PhysPageNum, VPNRange, VirtPageNum};

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1;
        const W = 1 << 1;
        const X = 1 << 2;
        const U = 1 << 3;
    }
}

impl From<MapPermission> for PteFlags {
    fn from(value: MapPermission) -> Self {
        let mut ret = PteFlags::P;
        if value.contains(MapPermission::W) {
            ret |= PteFlags::RW;
        }
        if value.contains(MapPermission::U) {
            ret |= PteFlags::US;
        }
        ret
    }
}

pub enum MapType {
    Identical,
    Framed,
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(vpn_range: VPNRange, map_type: MapType, map_perm: MapPermission) -> Self {
        Self { vpn_range, data_frames: BTreeMap::new(), map_type, map_perm }
    }

    fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range.start.0..self.vpn_range.end.0 {
            self.map_once(page_table, VirtPageNum(vpn));
        }
    }

    fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range.start.0..self.vpn_range.end.0 {
            self.unmap_once(page_table, VirtPageNum(vpn));
        }
    }

    fn map_once(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => ppn = PhysPageNum(vpn.0),
            MapType::Framed => {
                let frame = alloc_frame(1).unwrap();
                ppn = frame.base_ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        page_table.map(vpn, ppn, self.map_perm.into());
        match self.map_type {
            MapType::Framed => {
                vpn.get_bytes_array().iter_mut().for_each(|b| *b = 0);
            },
            MapType::Identical => (),
        }
    }

    fn unmap_once(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        match self.map_type {
            MapType::Framed => { self.data_frames.remove(&vpn); },
            _ => (),
        }
        page_table.unmap(vpn);
    }
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new(page_table: PageTable, areas: Vec<MapArea>) -> Self {
        Self { page_table, areas }
    }

    pub fn add(&mut self, mut area: MapArea) {
        area.map(&mut self.page_table);
        self.areas.push(area);
    }
}
