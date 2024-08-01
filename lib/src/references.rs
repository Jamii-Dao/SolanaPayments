use core::fmt;

use constant_time_eq::constant_time_eq_n;

use crate::RandomBytes;

#[derive(Clone, Copy)]
pub struct Reference<const N: usize>([u8; N]);

impl<const N: usize> Reference<N> {
    pub fn new() -> Self {
        let random = RandomBytes::<N>::new();

        Self(random.expose_owned())
    }

    pub fn to_hash(&self) -> blake3::Hash {
        blake3::hash(&self.0)
    }

    pub fn expose_owned(&self) -> [u8; N] {
        self.0
    }

    pub fn expose(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> fmt::Debug for Reference<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reference({})", self.to_hash())
    }
}

impl<const N: usize> fmt::Display for Reference<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hash().to_hex().as_str())
    }
}

impl<const N: usize> PartialEq for Reference<N> {
    fn eq(&self, other: &Self) -> bool {
        constant_time_eq_n(self.expose(), other.expose())
    }
}

impl<const N: usize> Eq for Reference<N> {}

impl<const N: usize> AsRef<[u8]> for Reference<N> {
    fn as_ref(&self) -> &[u8] {
        self.expose()
    }
}

impl<const N: usize> Default for Reference<N> {
    fn default() -> Self {
        Self::new()
    }
}
