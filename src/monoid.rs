use crate::semigroup::Semigroup;

pub trait Monoid {}

impl<T: Default + Semigroup> Monoid for T {}
