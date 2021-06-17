use core::ops;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LineStatus {
    bits: u8
}

impl LineStatus {
    pub const INPUT_FULL: Self = Self {
        bits: 1
    };

    pub const OUTPUT_EMPTY: Self = Self {
        bits: 1 << 5
    };

    pub const fn all() -> Self {
        Self {
            bits: Self::INPUT_FULL.bits | Self::OUTPUT_EMPTY.bits
        }
    }

    pub const fn bits(&self) -> u8 {
        self.bits
    }

    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self {
            bits: bits & Self::all().bits
        }
    }
}

impl ops::BitAnd for LineStatus {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits
        }        
    }
}

impl ops::BitAndAssign for LineStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl ops::BitOr for LineStatus {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits
        }        
    }
}

impl ops::BitOrAssign for LineStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl ops::BitXor for LineStatus {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits
        }        
    }
}

impl ops::BitXorAssign for LineStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}

