use crate::functor::Functor;

pub trait Pure: Functor {
    fn pure(t: Self::Of) -> Self::To<Self::Of>;
}
