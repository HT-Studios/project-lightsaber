use core::ops;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)] 
pub(in super) struct GlobalDescriptorTableEntryFlags {
    bits: u8
}

impl GlobalDescriptorTableEntryFlags {
    pub const NULL: Self = Self {
        bits: 0
    };

    pub const LONG_MODE: Self = Self {
        bits: 1 << 5
    };

    pub const PROTECTED_MODE: Self = Self {
        bits: 1 << 6
    };

    pub const fn bits(&self) -> u8 {
        self.bits
    }
}

impl ops::BitAnd for GlobalDescriptorTableEntryFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits
        }
    }
}

impl ops::BitAndAssign for GlobalDescriptorTableEntryFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl ops::BitOr for GlobalDescriptorTableEntryFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits
        }
    }
}

impl ops::BitOrAssign for GlobalDescriptorTableEntryFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl ops::BitXor for GlobalDescriptorTableEntryFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits
        }
    }
}

impl ops::BitXorAssign for GlobalDescriptorTableEntryFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}

