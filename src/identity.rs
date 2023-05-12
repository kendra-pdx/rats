use crate::{functor::Functor, hkt::HKT, applicative::Applicative, pure::Pure, monad::Monad, mt::MonadTrans};

struct IdentityT<M>(M);

impl<M: HKT> HKT for IdentityT<M> {
    type Of = M::Of;
    type To<B> = IdentityT<M::To<B>>;
}

impl <M: Pure> Pure for IdentityT<M> {
    fn pure(t: M::Of) -> Self::To<M::Of> {
        IdentityT(M::pure(t))
    }
}

impl<M: Functor> Functor for IdentityT<M> {
    fn fmap<F, B>(&self, f: F) -> Self::To<B>
    where
        F: Fn(&M::Of) -> B,
    {
        IdentityT(self.0.fmap(f))
    }
}

impl <M: Applicative> Applicative for IdentityT<M> {
    fn lift_a2<F, B, C>(&self, b: &Self::To<B>, f: F) -> Self::To<C>
    where
        F: Fn(&M::Of, &B) -> C {
        IdentityT(self.0.lift_a2(&b.0, f))
    }
}

impl <M: Monad> Monad for IdentityT<M> {
    fn bind<B, F>(self, f: F) -> Self::To<B>
    where
        F: Fn(Self::Of) -> Self::To<B> {
        IdentityT(self.0.bind(|x| f(x).0))
    }
}

impl <M: Monad> MonadTrans for IdentityT<M> {
    type Base = M;

    fn lift(base: Self::Base) -> Self {
        IdentityT(base)
    }
}