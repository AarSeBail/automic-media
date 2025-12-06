use std::marker::PhantomData;
use std::ops::Add;

use crate::tape::{Tape, TapeEntry};

pub struct Automaton<P, V, F>
where
    F: FnMut(&V) -> (P, V),
    V: Copy,
{
    head: Option<P>,
    transition: F,
    _p: PhantomData<V>,
}

impl<P, V, F> Automaton<P, V, F>
where
    P: Add<P, Output = P> + Copy,
    F: FnMut(&V) -> (P, V),
    V: Copy,
{
    pub fn new(transition: F) -> Self {
        Self {
            head: None,
            transition,
            _p: PhantomData,
        }
    }

    pub fn start(&mut self, head: P) {
        self.head = Some(head);
    }

    pub fn next<T: Tape<P, V>>(&mut self, tape: &mut T) -> Option<(P, V)> {
        if let Some(p) = self.head.take() {
            let value = tape.get_mut(&p);
            if let TapeEntry::Hit(v) = value {
                let (np, nv) = (self.transition)(&v);
                *v = nv;
                self.head = Some(np + p);
                Some((p, nv))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/*pub struct AutomatonIter<'a, P, V, F, T>
where
    P: Add<P, Output = P>,
    F: FnMut(&V) -> (P, V),
    V: Copy,
    T: Tape<P, V>,
    T: ?Sized,
{
    automata: &'a mut Automaton<P, V, F>,
    tape: &'a mut T,
}

impl<'a, P, V, F, T> Iterator for AutomatonIter<'a, P, V, F, T>
where
    P: Add<P, Output = P> + Copy,
    F: FnMut(&V) -> (P, V),
    V: Copy,
    T: Tape<P, V>,
{
    type Item = (P, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.automata.next(self.tape)
    }
}*/
