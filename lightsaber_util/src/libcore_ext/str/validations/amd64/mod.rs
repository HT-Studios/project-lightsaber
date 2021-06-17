use core::arch::x86_64::{
    __m128i,
    _mm_alignr_epi8,
    _mm_and_si128,
    _mm_cmpgt_epi8,
    _mm_loadu_si128,
    _mm_movemask_epi8,
    _mm_or_si128,
    _mm_setr_epi8,
    _mm_setzero_si128,
    _mm_set1_epi8,
    _mm_shuffle_epi8,
    _mm_srli_epi16,
    _mm_subs_epu8,
    _mm_testz_si128,
    _mm_xor_si128
};

use crate::libcore_ext::str::validations::helpers::UTF8ValidationAlgorithm;

type SIMDU8Value = super::helpers::SIMDU8Value<__m128i>;

impl SIMDU8Value {
    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn from_32_rem_leading(values: [u8; 32]) -> Self {
        Self::from(_mm_setr_epi8(
            values[16] as i8,
            values[17] as i8,
            values[18] as i8,
            values[19] as i8,
            values[20] as i8,
            values[21] as i8,
            values[22] as i8,
            values[23] as i8,
            values[24] as i8,
            values[25] as i8,
            values[26] as i8,
            values[27] as i8,
            values[28] as i8,
            values[29] as i8,
            values[30] as i8,
            values[31] as i8,
        ))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn repeat_16(values: [u8; 16]) -> Self {
        Self::from(_mm_setr_epi8(
            values[0] as i8,
            values[1] as i8,
            values[2] as i8,
            values[3] as i8,
            values[4] as i8,
            values[5] as i8,
            values[6] as i8,
            values[7] as i8,
            values[8] as i8,
            values[9] as i8,
            values[10] as i8,
            values[11] as i8,
            values[12] as i8,
            values[13] as i8,
            values[14] as i8,
            values[15] as i8,
        ))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn load_from_ptr(ptr: *const u8) -> Self {
        Self::from(_mm_loadu_si128(
            ptr.cast()
        ))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn lookup_16(self, values: [u8; 16]) -> Self {
        Self::from(_mm_shuffle_epi8(
            Self::repeat_16(
                values
            ).0,
            self.0
        ))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn splat(value: u8) -> Self {
        Self::from(_mm_set1_epi8(
            value as i8
        ))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn splat0() -> Self {
        Self::from(_mm_setzero_si128())
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn or(self, rhs: Self) -> Self {
        Self::from(_mm_or_si128(self.0, rhs.0))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn and(self, rhs: Self) -> Self {
        Self::from(_mm_and_si128(self.0, rhs.0))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn xor(self, rhs: Self) -> Self {
        Self::from(_mm_xor_si128(self.0, rhs.0))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn saturating_sub(self, rhs: Self) -> Self {
        Self::from(_mm_subs_epu8(self.0, rhs.0))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn shr4(self) -> Self {
        Self::from(_mm_srli_epi16::<4>(
            self.0,
        ))
            .and(Self::splat(0xFF >> 4))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn prev<const IMM8: i32>(self, prev: Self) -> Self
    where
        [(); const_fn_evaluatble_checked_workaround(16 - IMM8)]: , {
        Self::from(_mm_alignr_epi8::<{ 16 - IMM8 }>(
            self.0,
            prev.0
        ))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn signed_gt(&self, other: Self) -> Self {
        Self::from(_mm_cmpgt_epi8(self.0, other.0))
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn any_bit_set(self) -> bool {
        _mm_testz_si128(self.0, self.0) != 1
    }

    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn is_ascii(self) -> bool {
        _mm_movemask_epi8(self.0) == 0
    }
}

impl From<__m128i> for SIMDU8Value {
    #[inline]
    fn from(value: __m128i) -> Self {
        Self { 0: value }
    }
}

impl UTF8ValidationAlgorithm<SIMDU8Value> {
    #[inline]
    #[target_feature(enable = "sse4.2")]
    pub(in self) unsafe fn must_be_2_3_cont(prev2: SIMDU8Value, prev3: SIMDU8Value) -> SIMDU8Value {
        let third_byte = prev2.saturating_sub(SIMDU8Value::splat(0b1110_0000 - 1));
        let fourth_byte = prev3.saturating_sub(SIMDU8Value::splat(0b1110_0000 - 1));

        third_byte
            .or(fourth_byte)
            .signed_gt(SIMDU8Value::splat0())
    }
}

pub(in self) const fn const_fn_evaluatble_checked_workaround(val: i32) -> usize {
    val as usize
}
