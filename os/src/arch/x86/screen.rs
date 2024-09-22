use bitflags::bitflags;


bitflags! {
    pub struct ScreenCharAttr: u8 {
        const FOREGROUND_B = 1;
        const FOREGROUND_G = 1 << 1;
        const FOREGROUND_R = 1 << 2;
        const HIGHLIGHT = 1 << 3;
        const BACKGROUND_B = 1 << 4;
        const BACKGROUND_G = 1 << 5;
        const BACKGROUND_R = 1 << 6;
        const FLASH = 1 << 7;
    }
}

impl Default for ScreenCharAttr {
    fn default() -> Self {
        ScreenCharAttr::FOREGROUND_R | ScreenCharAttr::FOREGROUND_G | ScreenCharAttr::FOREGROUND_B
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScreenAttrChar {
    c: u8,
    attr: ScreenCharAttr,
}

impl ScreenAttrChar {
    pub fn new(c: u8, attr: ScreenCharAttr) -> Self {
        Self { c, attr }
    }
}

impl Default for ScreenAttrChar {
    fn default() -> Self {
        Self { c: ' ' as u8, attr: ScreenCharAttr::default() }
    }
}
