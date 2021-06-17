use core::mem;

#[inline]
pub const unsafe fn from_utf8_unchecked(slice: &[u8]) -> &str {
    mem::transmute(slice)
}

#[inline]
pub unsafe fn from_utf8_unchecked_mut(slice: &mut [u8]) -> &mut str {
    &mut *(slice as *mut [u8] as *mut str)
}
