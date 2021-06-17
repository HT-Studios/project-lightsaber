use core::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in super) struct SegmentSelector {
    bits: u16
}

impl SegmentSelector {
    pub const RPL_0: Self = Self {
        bits: 0b00
    };

    pub const RPL_1: Self = Self {
        bits: 0b01
    };

    pub const RPL_2: Self = Self {
        bits: 0b10
    };

    pub const RPL_3: Self = Self {
        bits: 0b11
    };

    pub(in super) const fn new(index: u16, rpl: Self) -> Self {
        Self {
            bits: index << 3 | rpl.bits
        }
    }

    pub const fn bits(&self) -> u16 {
        self.bits
    }
}

impl ops::BitAnd for SegmentSelector {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits
        }        
    }
}

impl ops::BitAndAssign for SegmentSelector {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl ops::BitOr for SegmentSelector {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits
        }        
    }
}

impl ops::BitOrAssign for SegmentSelector {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl ops::BitXor for SegmentSelector {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits
        }        
    }
}

impl ops::BitXorAssign for SegmentSelector {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}
