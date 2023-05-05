use crate::applicative::Applicative;

pub trait Monad: Applicative {
    fn bind<B, F>(&self, f: F) -> Self::To<B>
    where
        F: Fn(&Self::Of) -> Self::To<B>;
}
