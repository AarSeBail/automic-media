use std::marker::PhantomData;

use crate::Tape;

pub struct TapeMap<P, V, T: Tape<P, V>, F: Fn(&P) -> P> {
    base: T,
    map: F,
    _p: PhantomData<(P, V)>,
}

impl<P, V, T: Tape<P, V>, F: Fn(&P) -> P> TapeMap<P, V, T, F> {
    pub fn new(base: T, map: F) -> Self {
        Self {
            base,
            map,
            _p: PhantomData,
        }
    }
}

impl<P, V, T: Tape<P, V>, F: Fn(&P) -> P> Tape<P, V> for TapeMap<P, V, T, F> {
    fn get(&mut self, position: &P) -> super::TapeEntry<&V> {
        self.base.get(&(self.map)(position))
    }

    fn get_mut(&mut self, position: &P) -> super::TapeEntry<&mut V> {
        self.base.get_mut(&(self.map)(position))
    }
}
