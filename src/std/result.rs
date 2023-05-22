use crate::{applicative::Applicative, functor::Functor, hkt::HKT, monad::Monad, pure::Pure};

impl<A, E> HKT for Result<A, E> {
    type Of = A;
    type To<B> = Result<B, E>;
}

impl<A, E: Clone> Functor for Result<A, E> {
    fn fmap<F, B>(&self, f: F) -> Self::To<B>
    where
        F: Fn(&Self::Of) -> B,
    {
        match self {
            Ok(a) => Ok(f(a)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<A, E> Pure for Result<A, E> {
    fn pure(t: Self::Of) -> Self::To<Self::Of> {
        Ok(t)
    }
}

impl<A, E: Clone> Applicative for Result<A, E> {
    fn lift_a2<F, B, C>(&self, b: &Self::To<B>, f: F) -> Self::To<C>
    where
        F: Fn(&Self::Of, &B) -> C,
    {
        // self.as_ref().and_then(|a| b.map(move |b| f(a, &b)))
        match (self, b) {
            (Ok(a), Ok(b)) => Ok(f(a, &b)),
            (Err(e), _) => Err(e.clone()),
            (_, Err(e)) => Err(e.clone()),
        }
    }
}

impl<A, E: Clone> Monad for Result<A, E> {
    fn bind<B, F>(self, f: F) -> Self::To<B>
    where
        F: Fn(Self::Of) -> Self::To<B>,
    {
        match self {
            Ok(a) => f(a),
            Err(e) => Err(e.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{applicative::Applicative, functor::Functor, monad::Monad, pure::Pure};

    #[derive(Clone, PartialEq, Debug)]
    enum TestErr {
        Failure(),
    }

    #[test]
    fn fmap() {
        let r1: Result<i32, TestErr> = Ok(1);
        assert_eq!(Ok(2), r1.fmap(|&x| x + 1));

        let r2: Result<i32, TestErr> = Err(TestErr::Failure());
        assert_eq!(
            Err(TestErr::Failure()),
            r2.fmap(|_| panic!("should not reach"))
        )
    }

    #[test]
    fn pure() {
        assert_eq!(Ok(1), Result::<i32, TestErr>::pure(1))
    }

    #[test]
    fn ap() {
        let err: Result<i32, TestErr> = Err(TestErr::Failure());
        let init: Result<i32, TestErr> = Ok(1);
        let plus1: Result<fn(&i32) -> i32, TestErr> = Ok(|x| x + 1);
        assert_eq!(Ok(2), plus1.ap(&init));
        assert_eq!(err, plus1.ap(&err));
    }

    #[test]
    fn bind() {
        let err: Result<i32, TestErr> = Err(TestErr::Failure());
        let init: Result<i32, TestErr> = Ok(1);
        assert_eq!(Ok(2), init.bind(|x| Ok(x + 1)));
        assert_eq!(err, err.clone().bind(|_| panic!("should not be called")))
    }
}
