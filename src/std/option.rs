use crate::{
    applicative::Applicative, functor::Functor, hkt::derive_hkt, monad::Monad, pure::Pure,
};

derive_hkt!(Option);

impl<A> Functor for Option<A> {
    fn fmap<F: Fn(&A) -> B, B>(&self, f: F) -> Option<B> {
        match self {
            Some(x) => Some(f(&x)),
            None => None,
        }
    }
}

impl<A> Pure for Option<A> {
    fn pure(t: Self::Of) -> Self::To<Self::Of> {
        Some(t)
    }
}

impl<A> Applicative for Option<A> {
    fn lift_a2<F, B, C>(&self, b: &Self::To<B>, f: F) -> Self::To<C>
    where
        F: Fn(&Self::Of, &B) -> C,
    {
        self.as_ref().and_then(|a| b.as_ref().map(move |b| f(a, b)))
    }
}

impl<A> Monad for Option<A> {
    fn bind<B, F>(self, f: F) -> Self::To<B>
    where
        F: Fn(Self::Of) -> Self::To<B>,
    {
        self.and_then(|a| f(a))
    }
}

#[cfg(test)]
mod tests {
    use crate::std::option::*;

    #[test]
    fn pure() {
        assert_eq!(Some(1), Option::pure(1));
    }

    #[test]
    fn fmap() {
        assert_eq!(
            Some(String::from("a")),
            Some(String::from("A")).fmap(|s| s.to_lowercase())
        );
        assert_eq!(None, None.fmap(|s: &String| s.to_lowercase()));
    }

    #[test]
    fn lift_a2() {
        let a: Option<u32> = Some(1);
        let b: Option<u32> = Some(2);
        let c: Option<u32> = None;

        assert_eq!(Some(3), a.lift_a2(&b, |x, y| x + y));
        assert_eq!(None, c.lift_a2(&a, |x, y| x + y));
        assert_eq!(None, a.lift_a2(&c, |x, y| x + y));
        assert_eq!(None, c.lift_a2(&c, |x, y| x + y));
    }

    #[test]
    fn lift_a2_apply() {
        let a = Some(|x: &u32| x + 1);
        let b = Some(2);
        assert_eq!(Some(3), a.ap(&b))
    }

    #[test]
    fn bind() {
        assert_eq!(Some(2), Some(1).bind(|x| Some(x + 1)));
    }
}
