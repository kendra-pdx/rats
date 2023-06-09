use crate::pure::Pure;

pub trait Applicative: Pure {
    fn lift_a2<F, B, C>(&self, b: &Self::To<B>, f: F) -> Self::To<C>
    where
        F: Fn(&Self::Of, &B) -> C;

    fn ap<B, C>(&self, b: &Self::To<B>) -> Self::To<C>
    where
        Self::Of: Fn(&B) -> C,
    {
        self.lift_a2(b, |g, x| g(&x))
    }
}

// impl<T: Applicative> Functor for T {
//     fn fmap<F, B>(self, f: F) -> Self::To<B>
//     where
//         F: FnMut(Self::Of) -> B {
//         Self::pure(self).ap(f)
//     }

//     type Of=Self::Of;

//     type To<B>=Self::To<B>;
// }
