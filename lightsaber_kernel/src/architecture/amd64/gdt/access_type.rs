pub struct GlobalDescriptorTableAccessType;

impl GlobalDescriptorTableAccessType {
    pub const KERNEL_CODE: u16 = 1;
    pub const KERNEL_DATA: u16 = 2;
    pub const KERNEL_TLS: u16 = 3;
    pub const USER_CODE32_UNUSED: u16 = 4;
    pub const USER_TLS: u16 = 7;
    pub const TSS: u16 = 8;
    pub const TSS_HI: u16 = 9;
}

