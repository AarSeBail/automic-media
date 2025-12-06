pub mod mmap;

pub trait Store<V>
where
    Self: Sized,
{
    fn fill(size: usize, fill: V) -> Option<Self>;
    fn get(&mut self, position: usize) -> &V;
    fn get_mut(&mut self, position: usize) -> &mut V;
}

impl<V> Store<V> for Vec<V>
where
    V: Clone,
{
    fn fill(size: usize, fill: V) -> Option<Self> {
        Some(vec![fill; size])
    }

    fn get(&mut self, position: usize) -> &V {
        &self[position]
    }

    fn get_mut(&mut self, position: usize) -> &mut V {
        &mut self[position]
    }
}
