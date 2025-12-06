use std::marker::PhantomData;

use crate::{
    store::Store,
    tape::{Tape, TapeEntry},
    utils::kvec::KVec,
};

pub struct Grid<const N: usize, V, S: Store<V>> {
    upper: KVec<N>,
    lower: KVec<N>,
    size: KVec<N>,
    data: S,
    _p: PhantomData<V>,
}

impl<const N: usize, V: Copy, S: Store<V>> Grid<N, V, S> {
    pub fn fill(lower: KVec<N>, upper: KVec<N>, fill: V) -> Self {
        debug_assert!(lower <= upper, "lower > upper");
        let size = upper - lower + KVec([1; N]);
        let measure = size.0.iter().product::<i64>() as usize;
        Self {
            lower,
            upper,
            size,
            data: S::fill(measure, fill).unwrap(),
            _p: PhantomData,
        }
    }

    pub fn size(&self) -> KVec<N> {
        self.size
    }

    pub fn lower(&self) -> KVec<N> {
        self.lower
    }

    pub fn upper(&self) -> KVec<N> {
        self.upper
    }
}

impl<const N: usize, V: Copy, S: Store<V>> Tape<KVec<N>, V> for Grid<N, V, S> {
    fn get(&mut self, position: &KVec<N>) -> TapeEntry<&V> {
        if self.lower > *position {
            return TapeEntry::Miss;
        }
        if self.upper < *position {
            return TapeEntry::Miss;
        }
        let coord = *position - self.lower;

        let mut index = 0;
        for i in (0..N).rev() {
            index *= self.size.0[i];
            index += coord.0[i];
        }
        TapeEntry::Hit(self.data.get(index as usize))
    }

    fn get_mut(&mut self, position: &KVec<N>) -> TapeEntry<&mut V> {
        if self.lower > *position {
            return TapeEntry::Miss;
        }
        if self.upper < *position {
            return TapeEntry::Miss;
        }
        let coord = *position - self.lower;

        let mut index = 0;
        for i in (0..N).rev() {
            index *= self.size.0[i];
            index += coord.0[i];
        }
        TapeEntry::Hit(self.data.get_mut(index as usize))
    }
}
