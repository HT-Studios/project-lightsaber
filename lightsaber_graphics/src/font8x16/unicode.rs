use core::slice;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnicodeFont(pub char, pub [u8; 16]);

impl UnicodeFont {
    pub fn char(&self) -> char {
        self.0
    }

    pub fn bytearray(&self) -> [u8; 16] {
        self.1
    }

    pub fn is_whitespace(&self) -> bool {
        self.1 == crate::font8x16::legacy::NOTHING
    }

    pub fn into_inner(self) -> (char, [u8; 16]) {
        self.into()
    }
}

impl From<UnicodeFont> for [u8; 16] {
    fn from(f: UnicodeFont) -> Self {
        f.1
    }
}

impl From<UnicodeFont> for char {
    fn from(f: UnicodeFont) -> Self {
        f.0
    }
}

impl From<UnicodeFont> for (char, [u8; 16]) {
    fn from(f: UnicodeFont) -> Self {
        (f.0, f.1)
    }
}

pub trait UnicodeFonts {
    fn get(&self, key: char) -> Option<[u8; 16]>;

    fn get_font(&self, key: char) -> Option<UnicodeFont>;

    fn iter(&self) -> slice::Iter<UnicodeFont>;
}
