pub(in super) struct GlobalDescriptorTableAccessFlags;

impl GlobalDescriptorTableAccessFlags {
    pub(in super) const NULL: u8 = 0;
    pub(in super) const PRESENT: u8 = 1 << 7;
    pub(in super) const RING_0: u8 = 0 << 5;
    pub(in super) const RING_3: u8 = 3 << 5;
    pub(in super) const SYSTEM: u8 = 1 << 4;
    pub(in super) const EXECUTABLE: u8 = 1 << 3;
    pub(in super) const PRIVILEGE: u8 = 1 << 1;
    pub(in super) const TSS_AVAIL: u8 = 9;
}
