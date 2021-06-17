use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UTF8Error {
    pub(in crate) valid_up_to: usize,
    pub(in crate) error_length: Option<u8>
}

impl UTF8Error {
    #[inline]
    pub const fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }

    #[inline]
    pub fn error_length(&self) -> Option<usize> {
        self.error_length.map(|length| length as usize)
    }
}

impl fmt::Display for UTF8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.error_length.is_some() {
            return write!(
                f,
                "incomplete UTF8 byte sequence starting from byte index {}",
                self.valid_up_to
            );
        }

        write!(
            f,
            "invalid UTF8 byte sequence of length {} bytes, starting from byte index {}",
            self.error_length.unwrap(),
            self.valid_up_to
        )
    }
}
