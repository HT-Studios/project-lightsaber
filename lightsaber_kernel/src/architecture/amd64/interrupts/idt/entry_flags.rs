use core::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InterruptDescriptorTableFlags {
    pub bits: u8
}

impl InterruptDescriptorTableFlags {
    pub const PRESENT: Self = Self {
        bits: 1 << 7
    };

    pub const RING_0: Self = Self {
        bits: 0 << 5
    };

    pub const RING_1: Self = Self {
        bits: 1 << 5
    };

    pub const RING_2: Self = Self {
        bits: 2 << 5
    };

    pub const RING_3: Self = Self {
        bits: 3 << 5
    };

    pub const SS: Self = Self {
        bits: 1 << 4
    };

    pub const INTERRUPT: Self = Self {
        bits: 0xE
    };

    pub const TRAP: Self = Self {
        bits: 0xF
    };
}

impl ops::BitAnd for InterruptDescriptorTableFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits
        }
    }
}

impl ops::BitAndAssign for InterruptDescriptorTableFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl ops::BitOr for InterruptDescriptorTableFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits
        }
    }
}

impl ops::BitOrAssign for InterruptDescriptorTableFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl ops::BitXor for InterruptDescriptorTableFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits
        }
    }
}

impl ops::BitXorAssign for InterruptDescriptorTableFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}
