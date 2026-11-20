//! Stands in for an arvo crate declaring `Constant` over a const parameter of
//! the stack's own type rather than a bare `i128`. `Index` has a private field,
//! the shape `arvo_format::Slot` and `Width` have, which is the case the
//! ruling on the numeric door says `adt_const_params` accepts. The gate is in
//! this crate only.

#![no_std]
#![feature(adt_const_params)]

use core::marker::ConstParamTy;

#[derive(ConstParamTy, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Index(i128);

impl Index {
    pub const fn at(i: i128) -> Self {
        Self(i)
    }

    pub const fn get(self) -> i128 {
        self.0
    }
}

pub trait Constant<const V: Index>: Sized {
    const MEMBER: Self;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Small(pub Index);

impl<const V: Index> Constant<V> for Small {
    const MEMBER: Self = {
        assert!(V.get() >= 0 && V.get() < 256, "no such member");
        Small(V)
    };
}
