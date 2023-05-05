
pub trait Kind {

}

pub trait HKT {
    type Of;
    type To<B>;
}

macro_rules! derive_hkt {
    ($t:ident) => {
        impl<T> crate::hkt::HKT for $t<T> {
            type Of = T;
            type To<B> = $t<B>;
        }
    }
}

pub(crate) use derive_hkt;