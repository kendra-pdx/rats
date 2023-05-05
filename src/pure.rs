use crate::hkt::HKT;

pub trait Pure: HKT {
    fn pure(t: Self::Of) -> Self::To<Self::Of>;
}
