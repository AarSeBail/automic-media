use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

use crate::tape::{Tape, TapeEntry};

pub struct TapeExtension<P: Eq + Hash + Copy + Add<P, Output = P>, V: Copy, T: Tape<P, V>> {
    base: T,
    store: HashMap<P, V>,
    fill: V,
}

impl<P: Eq + Hash + Copy + Add<P, Output = P>, V: Copy, T: Tape<P, V>> TapeExtension<P, V, T> {
    pub fn base(self) -> T {
        self.base
    }
}

impl<P: Eq + Hash + Copy + Add<P, Output = P>, V: Copy, T: Tape<P, V>> Tape<P, V>
    for TapeExtension<P, V, T>
{
    // Perhaps this should use interior mutability to insert on read
    fn get(&mut self, position: &P) -> TapeEntry<&V> {
        match self.base.get(position) {
            TapeEntry::Hit(entry) => return TapeEntry::Hit(entry),
            TapeEntry::Miss => {}
        }
        let entry = self.store.entry(*position).or_insert(self.fill);
        TapeEntry::Hit(entry)
    }

    fn get_mut(&mut self, position: &P) -> TapeEntry<&mut V> {
        match self.base.get_mut(position) {
            TapeEntry::Hit(entry) => return TapeEntry::Hit(entry),
            TapeEntry::Miss => {}
        }
        let entry = self.store.entry(*position).or_insert(self.fill);
        TapeEntry::Hit(entry)
    }
}

pub fn extend_tape<P: Eq + Hash + Copy + Add<P, Output = P>, V: Copy, T: Tape<P, V>>(
    base: T,
    fill: V,
) -> TapeExtension<P, V, T> {
    TapeExtension {
        base,
        store: HashMap::new(),
        fill,
    }
}
