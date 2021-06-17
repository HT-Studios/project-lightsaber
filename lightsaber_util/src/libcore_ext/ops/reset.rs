pub trait Reset {
    type Output;

    fn reset(&mut self) -> Self::Output;
}
