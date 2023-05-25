use crate::{applicative::Applicative, hkt::HKT, pure::Pure, semigroup::Semigroup};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Validation<A, E> {
    Ok(A),
    Err(E),
}

impl<A, E> HKT for Validation<A, E> {
    type Of = A;
    type To<B> = Validation<B, E>;
}

impl <A, E> From<Result<A, E>> for Validation<A, E> {
    fn from(value: Result<A, E>) -> Self {
        match value {
            Ok(v) => Validation::Ok(v),
            Err(e) => Validation::Err(e)
        }
    }
}

impl <A, E> From<Validation<A, E>> for Result<A,E> {
    fn from(value: Validation<A, E>) -> Self {
        match value {
            Validation::Err(e) => Err(e),
            Validation::Ok(v) => Ok(v)
        }
    }
}

impl<A, E> Pure for Validation<A, E> {
    fn pure(t: Self::Of) -> Self::To<Self::Of> {
        Validation::Ok(t)
    }
}

impl<A, E: Semigroup + Clone> Applicative for Validation<A, E> {
    fn lift_a2<F, B, C>(&self, b: &Self::To<B>, f: F) -> Self::To<C>
    where
        F: Fn(&Self::Of, &B) -> C,
    {
        match (self, b) {
            (Validation::Ok(a), Validation::Ok(b)) => Validation::Ok(f(a, b)),
            (Validation::Err(e), Validation::Ok(_)) => Validation::Err(e.clone()),
            (Validation::Ok(_), Validation::Err(e)) => Validation::Err(e.clone()),
            (Validation::Err(e1), Validation::Err(e2)) => {
                Validation::Err(e1.clone().combine(e2.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Validation;
    use crate::applicative::Applicative;

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Errors {
        TestError1(),
        TestError2(),
    }

    type TestValidation = Validation<i32, Vec<Errors>>;

    fn add(a: &i32, b: &i32) -> i32 {
        a + b
    }

    #[test]
    fn lift_a2() {
        let ok1: TestValidation = Validation::Ok(1);
        let ok2: TestValidation = Validation::Ok(2);
        let err1: TestValidation = Validation::Err(vec![Errors::TestError1()]);
        let err2: TestValidation = Validation::Err(vec![Errors::TestError2()]);

        let r = ok1.lift_a2(&ok2, add);
        assert_eq!(Validation::Ok(3), r);
        assert_eq!(err1, ok1.lift_a2(&err1, add));
        assert_eq!(err2, err2.lift_a2(&ok1, add));
        assert_eq!(
            Validation::Err(vec![Errors::TestError1(), Errors::TestError2()]),
            err1.lift_a2(&err2, add)
        );
    }

    #[test]
    fn from() {
        let ok1: TestValidation = Validation::Ok(1);
        assert_eq!(Ok(1), Result::from(ok1));

        let err1: TestValidation = Validation::Err(vec![Errors::TestError1()]);
        assert_eq!(Err(vec![Errors::TestError1()]), Result::from(err1));

        let ok2: Result<i32, Vec<Errors>> = Ok(2);
        assert_eq!(Validation::Ok(2), Validation::from(ok2));

        let err2: Result<i32, Vec<Errors>> = Err(vec![Errors::TestError2()]);
        assert_eq!(Validation::Err(vec![Errors::TestError2()]), Validation::from(err2));
    }
}
