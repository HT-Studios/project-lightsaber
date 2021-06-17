use crate::libcore_ext::ops::Reset;

impl Reset for usize {
    type Output = usize;

    fn reset(&mut self) -> Self::Output {
        *self = Default::default();
        *self
    }
}
