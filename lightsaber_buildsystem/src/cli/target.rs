use std::ops;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetTriple(pub String);

impl TargetTriple {
    pub fn new(triple: &str) -> Self {
        Self(triple.to_string())
    }
}

impl ops::Deref for TargetTriple {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
