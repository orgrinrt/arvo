//! P7. Can an ambient strategy be supplied for a lexical region by name
//! resolution alone, with no AST rewriting of the consumer's spellings?
//!
//! The mechanism under test: the strategy-elided spellings are per-strategy
//! alias sets, and a scope selects one by importing it. Four things are
//! checked at once.
//!
//! 1. A module-scope import supplies the ambient strategy for the module.
//! 2. An inner-scope import overrides it for that scope, and nesting works
//!    because lexical scopes nest.
//! 3. A fully written spelling naming its own strategy is unaffected by any
//!    enclosing import.
//! 4. A type alias defined outside the scope keeps its own site's resolution,
//!    which P6 shows is required rather than merely acceptable.
//!
//! Expected: all four hold, with no proc macro anywhere in the file.

#![no_std]

pub mod strategies {
    pub struct Hot;
    pub struct Warm;
    pub struct Precise;
}

pub mod base {
    /// The full spelling. Every parameter present, nothing elided.
    pub struct UInt<const N: u32, S>(pub u32, core::marker::PhantomData<S>);

    impl<const N: u32, S> UInt<N, S> {
        pub const fn new(v: u32) -> Self {
            Self(v, core::marker::PhantomData)
        }
    }
}

/// One alias set per strategy. The elided spelling is a different item in
/// each, so selecting a set is selecting a strategy.
pub mod warm {
    pub type UInt<const N: u32> = crate::base::UInt<N, crate::strategies::Warm>;
}
pub mod hot {
    pub type UInt<const N: u32> = crate::base::UInt<N, crate::strategies::Hot>;
}

// (1) The module's ambient strategy.
use warm::UInt;

/// A domain alias, defined at module scope, under the module's ambient
/// strategy. Tier two writes this once.
pub type StrHandle = UInt<5>;

/// (1) The elided spelling here is the module's ambient strategy.
pub fn ambient(v: u32) -> base::UInt<5, strategies::Warm> {
    UInt::<5>::new(v)
}

/// (2) An inner scope selects a different set. The same elided spelling now
/// denotes the other strategy, and nothing was rewritten.
pub fn scoped(v: u32) -> base::UInt<5, strategies::Hot> {
    use hot::UInt;
    UInt::<5>::new(v)
}

/// (2) Nesting: the innermost selection wins, and the outer one is restored
/// on the way out.
pub fn nested(
    v: u32,
) -> (
    base::UInt<5, strategies::Hot>,
    base::UInt<5, strategies::Warm>,
) {
    let inner = {
        use hot::UInt;
        UInt::<5>::new(v)
    };
    let outer = UInt::<5>::new(v);
    (inner, outer)
}

/// (3) An explicit spelling names its own strategy and the enclosing scope
/// does not reach it.
pub fn explicit_wins(v: u32) -> base::UInt<5, strategies::Precise> {
    use hot::UInt as _Ambient;
    base::UInt::<5, strategies::Precise>::new(v)
}

/// (4) The domain alias carries its definition site's resolution into a scope
/// with a different ambient strategy. It stays Warm, which P6 shows it must.
pub fn alias_keeps_its_site(v: u32) -> base::UInt<5, strategies::Warm> {
    use hot::UInt;
    let _local: base::UInt<5, strategies::Hot> = UInt::<5>::new(v);
    StrHandle::new(v)
}
