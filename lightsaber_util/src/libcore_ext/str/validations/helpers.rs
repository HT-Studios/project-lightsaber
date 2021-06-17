#[derive(Clone, Copy)]
pub(in super) struct SIMDU8Value<T>(pub(in super) T)
where
    T: Copy;

#[repr(C, align(32))]
pub(in super) struct UTF8ValidationAlgorithm<T> {
    pub(in super) previous: T,
    pub(in super) incomplete: T,
    pub(in super) error: T
}
