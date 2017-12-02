/// After original iterator ends, yields first element once again.
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct CircularState<I> {
    iter: I,
    orig: I,
    consumed: bool,
}

impl<I> Iterator for CircularState<I> where I: Clone + Iterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        match self.iter.next() {
            None if self.consumed == true => None,
            None => {
                // TODO: It seems redundant to have else one copy for one element only
                let mut iter = self.orig.clone();
                self.consumed = true;
                iter.next()
            },
            y => y
        }
    }
}

pub trait Circular: Iterator {
    #[inline]
    fn circular(self) -> CircularState<Self>
        where Self: Sized + Clone {
            CircularState {
                iter: self.clone(),
                orig: self,
                consumed: false,
            }
    }
}

impl<T: Iterator> Circular for T {}

