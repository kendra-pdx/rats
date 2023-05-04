pub trait Semigroup {
    fn combine(self, rhs: Self) -> Self;
}
