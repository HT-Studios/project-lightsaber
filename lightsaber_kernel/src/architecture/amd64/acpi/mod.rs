#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TableSignature([u8; 4]);

impl TableSignature {
    pub const DSDT: Self = Self(*b"DSDT");
    pub const FADT: Self = Self(*b"FADT");
    pub const GTDT: Self = Self(*b"GTDT");
    pub const MADT: Self = Self(*b"MADT");
    pub const MCFG: Self = Self(*b"MCFG");
    pub const RSDP: Self = Self(*b"RSDP");
    pub const SPCR: Self = Self(*b"SPCR");
    pub const XSDT: Self = Self(*b"XSDT");
}
