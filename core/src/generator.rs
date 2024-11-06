use std::ops::{Deref, DerefMut};

pub trait Generator {
    type State: Default;
    type Item: ?Sized;

    fn next(&self, iter_state: &mut Self::State) -> Option<&Self::Item>;
}

pub trait GeneratorMut {
    type State: Default;
    type Item: ?Sized;

    fn next_mut(&mut self, iter_state: &mut Self::State) -> Option<&mut Self::Item>;
}

impl<T, U> Generator for T
where
    T: Deref<Target = [U]>,
{
    type State = usize;
    type Item = U;

    fn next(&self, iter_state: &mut Self::State) -> Option<&Self::Item> {
        let Some(out) = self.deref().get(*iter_state) else {
            return None;
        };
        *iter_state += 1;
        Some(out)
    }
}

impl<T, U> GeneratorMut for T
where
    T: DerefMut<Target = [U]>,
{
    type State = usize;
    type Item = U;

    fn next_mut(&mut self, iter_state: &mut Self::State) -> Option<&mut Self::Item> {
        let Some(out) = self.deref_mut().get_mut(*iter_state) else {
            return None;
        };
        *iter_state += 1;
        Some(out)
    }
}
