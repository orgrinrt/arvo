// Can a type declared in a DOWNSTREAM crate (which is what a macro
// expansion produces, textually, in the CALLING crate's compilation unit)
// ever implement a trait sealed via a private supertrait in an UPSTREAM
// crate? This is the tower-crate half of the cross-crate seal test; run
// against a copy of tower.rs's actual sealing shape, not a toy.
mod tower {
    mod sealed {
        pub trait Sealed {}
    }
    pub trait Bias: sealed::Sealed {
        const NUM: i128;
    }
    pub struct BZero;
    impl sealed::Sealed for BZero {}
    impl Bias for BZero {
        const NUM: i128 = 0;
    }
}

// Stand-in for "code a macro expansion would insert into a downstream
// crate": no access to `tower::sealed` at all, because it was never `pub`.
struct MintedByMacro;
impl tower::Bias for MintedByMacro {
    const NUM: i128 = 37;
}

fn main() {}
