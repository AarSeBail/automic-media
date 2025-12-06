use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KVec<const N: usize>(pub [i64; N]);

impl<const N: usize> KVec<N> {
    pub const ZERO: Self = Self([0; N]);
}

impl<const N: usize> PartialOrd for KVec<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.0[0].cmp(&other.0[0]);
        for i in 1..N {
            if cmp != self.0[i].cmp(&other.0[i]) {
                return None;
            }
        }
        Some(cmp)
    }
}

impl<const N: usize> Add<KVec<N>> for KVec<N> {
    type Output = KVec<N>;

    fn add(mut self, rhs: KVec<N>) -> Self::Output {
        for i in 0..N {
            self.0[i] += rhs.0[i];
        }
        self
    }
}

impl<const N: usize> Sub<KVec<N>> for KVec<N> {
    type Output = KVec<N>;

    fn sub(mut self, rhs: KVec<N>) -> Self::Output {
        for i in 0..N {
            self.0[i] -= rhs.0[i];
        }
        self
    }
}
