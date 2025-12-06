use crate::map::TapeMap;

pub mod extend;
pub mod grid;
pub mod map;

pub enum TapeEntry<V> {
    Hit(V),
    Miss,
}

pub trait Tape<Position, Value> {
    fn get(&mut self, position: &Position) -> TapeEntry<&Value>;
    fn get_mut(&mut self, position: &Position) -> TapeEntry<&mut Value>;

    fn map<F: Fn(&Position) -> Position>(self, map: F) -> TapeMap<Position, Value, Self, F>
    where
        Self: Sized,
    {
        TapeMap::new(self, map)
    }
}

impl<P, V, T> Tape<P, V> for &mut T
where
    T: Tape<P, V>,
{
    fn get(&mut self, position: &P) -> TapeEntry<&V> {
        T::get(self, position)
    }

    fn get_mut(&mut self, position: &P) -> TapeEntry<&mut V> {
        T::get_mut(self, position)
    }
}
