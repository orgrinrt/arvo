//! Const-context smoke tests for the `bitfield!` macro's emitted
//! `ConstEq` and `ConstDefault` impls.
//!
//! Round 5 (#315) added `impl const ConstPartialEq` /
//! `ConstEq` / `ConstDefault` to the macro's emission. These
//! tests exercise the const surface across three sample bitfield
//! shapes: a 1-bit slot, a multi-bit slot inside a u64 parent, and
//! a multi-bit slot inside a u32 parent.

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]

use arvo::bitfield;
use arvo::{Bool, ConstDefault, ConstEq, ConstPartialEq};

bitfield! {
    /// 1-bit slot inside an 8-bit parent (smallest meaningful shape).
    pub struct OneBit: 8 {
        /// The single flag.
        flag: 1 at 0,
    }
}

bitfield! {
    /// Multi-bit slot inside a 64-bit parent (the upper container).
    pub struct PackedHandle: 64 {
        /// 8-bit kind tag.
        kind: 8 at 56,
        /// 56-bit identity payload.
        id: 56 at 0,
    }
}

bitfield! {
    /// Multi-bit slot inside a 32-bit parent (mid-width container).
    pub struct StrHandle: 32 {
        /// 1 = runtime-interned, 0 = compile-time.
        origin: 1 at 31,
        /// Reserved flag bits.
        reserved: 3 at 28,
        /// 28-bit interned identity.
        id: 28 at 0,
    }
}

const _ONEBIT_CONST_DEFAULT_IS_ZERO: () = {
    let d: OneBit = <OneBit as ConstDefault>::const_default();
    let z: OneBit = OneBit::new();
    let eq = <OneBit as ConstPartialEq>::const_eq(&d, &z);
    assert!(eq.0);
};

const _PACKEDHANDLE_CONST_DEFAULT_IS_ZERO: () = {
    let d: PackedHandle = <PackedHandle as ConstDefault>::const_default();
    let z: PackedHandle = PackedHandle::new();
    let eq = <PackedHandle as ConstPartialEq>::const_eq(&d, &z);
    assert!(eq.0);
};

const _STRHANDLE_CONST_DEFAULT_IS_ZERO: () = {
    let d: StrHandle = <StrHandle as ConstDefault>::const_default();
    let z: StrHandle = StrHandle::new();
    let eq = <StrHandle as ConstPartialEq>::const_eq(&d, &z);
    assert!(eq.0);
};

const _STRHANDLE_DIFFERENT_VALUES_NOT_EQ: () = {
    let a: StrHandle = StrHandle::new();
    let b: StrHandle = StrHandle::new().with_id(arvo::Bits::<28, arvo::Hot>::from_raw(7u32));
    let eq = <StrHandle as ConstPartialEq>::const_eq(&a, &b);
    assert!(!eq.0);
};

#[test]
fn const_eq_marker_is_present() {
    // Trait coherence: ConstEq is an empty marker that requires
    // ConstPartialEq. Verify both impls are reachable.
    fn requires_const_eq<T: ConstEq>() {}
    requires_const_eq::<OneBit>();
    requires_const_eq::<PackedHandle>();
    requires_const_eq::<StrHandle>();
}

#[test]
fn const_default_runtime_matches_new() {
    let d: StrHandle = <StrHandle as ConstDefault>::const_default();
    let z: StrHandle = StrHandle::new();
    assert_eq!(d.to_bits().to_raw(), z.to_bits().to_raw());
}
