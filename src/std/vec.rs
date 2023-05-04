use crate::{
    applicative::Applicative, functor::Functor, monad::Monad, pure::Pure, semigroup::Semigroup, hkt::derive_hkt,
};

derive_hkt!(Vec);

impl<A> Functor for Vec<A> {
    fn fmap<F, B>(self, f: F) -> Self::To<B>
    where
        F: FnMut(Self::Of) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A> Semigroup for Vec<A> {
    fn combine(self, rhs: Self) -> Self {
        self.into_iter().chain(rhs.into_iter()).collect()
    }
}

impl<A> Pure for Vec<A> {
    fn pure(t: Self::Of) -> Self::To<Self::Of> {
        vec![t]
    }
}

fn product<'a: 'c, 'b: 'c, 'c, A, B>(
    xs: &'a Vec<A>,
    ys: &'b Vec<B>,
) -> impl Iterator<Item = (&'a A, &'b B)> + 'c {
    xs.iter().flat_map(move |x| std::iter::repeat(x).zip(ys))
}

impl<A> Applicative for Vec<A> {
    fn lift_a2<F, B, C>(self, b: Self::To<B>, mut f: F) -> Self::To<C>
    where
        F: FnMut(Self::Of, B) -> C,
        Self::Of: Copy,
        B: Copy,
    {
        product(&self, &b).map(|(a, b)| f(*a, *b)).collect()
    }
}

impl<A> Monad for Vec<A> {
    fn bind<B, F>(self, f: F) -> Self::To<B>
    where
        F: Fn(Self::Of) -> Self::To<B>,
    {
        self.into_iter().flat_map(f).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        applicative::Applicative, functor::Functor, monad::Monad, pure::Pure, semigroup::Semigroup,
    };

    #[test]
    fn fmap() {
        assert_eq!(vec![2, 3, 4], vec![1, 2, 3].fmap(|x| x + 1))
    }

    #[test]
    fn combine() {
        assert_eq!(vec![1, 2, 3, 4, 5], vec![1, 2].combine(vec![3, 4, 5]))
    }

    #[test]
    fn pure() {
        assert_eq!(vec![1], Vec::pure(1))
    }

    #[test]
    fn ap() {
        let plus1 = |x: i32| x + 1;
        let plus2 = |x: i32| x + 2;
        let a = vec![plus1, plus2];
        let b = vec![1, 2];
        assert_eq!(vec![2, 3, 3, 4], a.ap(b))
    }

    #[test]
    fn bind() {
        let plus = |x: i32| vec![x + 1, x + 2];
        let xs = vec![1, 2];
        let r = vec![2, 3, 3, 4];
        assert_eq!(r, xs.bind(plus));
    }

    #[test]
    fn bind_empty() {
        let empty = |_: i32| vec![];
        let xs = vec![1, 2];
        let r: Vec<i32> = vec![];
        assert_eq!(r, xs.bind(empty));
    }
}
