// p4c: where a declaration-site refusal has to live to be one.
//
// Three placements of the same assertion, against a declaration that violates
// it and is never constructed:
//   A. an associated const in the inherent impl, mentioned by no one
//   B. an associated const mentioned by a `const fn` in the same impl
//   C. a free anonymous const item emitted beside the type
//
// Only B and C fire. A is silent, which makes A's guarantee conditional on an
// unrelated line elsewhere in the emission.

macro_rules! shape_a {
    ($name:ident, $occ:literal, $n:literal) => {
        pub struct $name(u32);
        impl $name {
            const _FITS: () = {
                assert!($occ <= $n, "A: fields exceed container");
            };
            pub const fn zero() -> Self {
                Self(0)
            }
        }
    };
}

macro_rules! shape_b {
    ($name:ident, $occ:literal, $n:literal) => {
        pub struct $name(u32);
        impl $name {
            const _FITS: () = {
                assert!($occ <= $n, "B: fields exceed container");
            };
            pub const fn zero() -> Self {
                let _ = Self::_FITS;
                Self(0)
            }
        }
    };
}

macro_rules! shape_c {
    ($name:ident, $occ:literal, $n:literal) => {
        pub struct $name(u32);
        impl $name {
            pub const fn zero() -> Self {
                Self(0)
            }
        }
        const _: () = {
            assert!($occ <= $n, "C: fields exceed container");
        };
    };
}

shape_a!(ViolatesA, 33, 32);
// shape_b!(ViolatesB, 33, 32);   // uncomment: refuses
// shape_c!(ViolatesC, 33, 32);   // uncomment: refuses

fn main() {}
