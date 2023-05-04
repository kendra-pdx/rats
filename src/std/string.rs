use crate::{semigroup::Semigroup, monoid::Monoid};

impl Semigroup for String {
    fn combine(self, rhs: Self) -> Self {
        self + &rhs
    }
}

impl Monoid for String {
}

#[cfg(test)]
mod tests {
    use crate::semigroup::Semigroup;

    #[test]
    fn semigroup_combine() {
        assert_eq!(
            String::from("ab"),
            String::from("a").combine(String::from("b"))
        )
    }

    #[test]
    fn monoid() {
        assert_eq!(
            String::from("ab"),
            String::default().combine(String::from("a")).combine(String::from("b"))
        )
    }
}
