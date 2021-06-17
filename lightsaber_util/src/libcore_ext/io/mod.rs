pub const INTEL_ARCHITECTURE_32BIT_FS_BASE: u32 = 0xC0000100;

pub const INTEL_ARCHITECTURE_32BIT_GS_BASE: u32 = 0xC0000101;
pub const INTEL_ARCHITECTURE_32BIT_KERNEL_GS_BASE: u32 = 0xC0000102;

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let return_value: u8;

    asm!(
        "in al, dx",
        in("dx") port,
        out("al") return_value
    );

    return_value
}

#[inline]
pub unsafe fn inl(port: u16) -> u32 {
    let return_value: u32;

    asm!(
        "in eax, dx",
        in("dx") port,
        out("eax") return_value
    );

    return_value
}

#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value
    );
}

#[inline]
pub unsafe fn outl(port: u16, value: u32) {
    asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") value
    );
}

#[inline]
pub unsafe fn wait() {
    outb(0x80, 0)
}

#[inline]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let (high, low): (u32, u32);

    asm!(
        "rdmsr",
        out("eax") low,
        out("edx") high,
        in("ecx") msr,
        options(nomem)
    );

    ((high as u64) << 32) | (low as u64)
}

#[inline]
pub unsafe fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;

    asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low,
        in("edx") high,
        options(nomem)
    );
}
