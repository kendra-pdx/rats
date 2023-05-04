pub trait Functor {
    type Of;
    type To<B>: Functor;

    fn fmap<F, B>(self, f: F) -> Self::To<B>
    where
        F: FnMut(Self::Of) -> B;
}
