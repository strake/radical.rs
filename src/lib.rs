#![feature(core_float)]

#![no_std]

#[cfg(feature = "dimensioned")]
extern crate dimensioned as dim;
extern crate typenum;

#[allow(unused)]
use core::marker::PhantomData;
use core::num::Float;
use typenum::{ NonZero, PInt, Unsigned };

pub trait Radical<N: Unsigned> {
    type Root;
    fn root(self) -> Self::Root;
}

macro_rules! impl_Radical_float {
    ($T: ty) => (impl<N: Unsigned> Radical<N> for $T {
        type Root = $T;
        #[inline] fn root(self) -> $T { self.powi(N::to_usize() as i32).recip() }
    })
}

impl_Radical_float!(f32);
impl_Radical_float!(f64);

#[cfg(feature = "dimensioned")]
impl<N: Unsigned + NonZero, D: dim::Dimension + dim::Root<PInt<N>>, A: Radical<N>> Radical<N> for dim::Dim<D, A> where D::Output: dim::Dimension {
    type Root = dim::Dim<<D as dim::Root<PInt<N>>>::Output, A::Root>;
    #[inline] fn root(self) -> Self::Root { dim::Dim(self.0.root(), PhantomData) }
}
