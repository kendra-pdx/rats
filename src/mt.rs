use crate::monad::Monad;

pub trait MonadTrans {
    type Base: Monad;
    fn lift(base: Self::Base) -> Self;
}

#[allow(dead_code)]
pub fn join<MOuter, MInner, A>(outer: MOuter) -> MOuter::To<A>
where
    MOuter: Monad<Of = MInner, To<A> = MInner>,
    MInner: Monad<Of = A, To<A> = MOuter::To<A>>,
    
{
    outer.bind::<A, _>(|inner| inner)
}

#[cfg(test)]
mod tests {
    use crate::mt::join;

    #[test]
    fn join_options() {
        assert_eq!(Some(1), join(Some(Some(1))));
        assert_eq!(None as Option<i32>, join(Some(None)));
        assert_eq!(None as Option<i32>, join(None));
    }
}