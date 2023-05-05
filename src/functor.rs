use crate::hkt::HKT;

pub trait Functor: HKT {
    fn fmap<F, B>(self, f: F) -> Self::To<B>
    where
        F: Fn(&Self::Of) -> B;
}
