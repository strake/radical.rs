#![feature(core_float)]

#![no_std]

extern crate typenum;

use core::num::Float;
use typenum::Unsigned;

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
