//! The same carry with the invariant proof.
use core::marker::PhantomData;

use a_proof_across_a_subtyping_coercion::{Invariant, Narrow, Wide, bit_invariant};

fn main() {
    let proof: Invariant<Wide, 100> = Invariant::NEW;
    let carried: Invariant<Narrow, 100> = proof;
    let narrow: Narrow = Narrow {
        bytes: &[0xFF],
        kind:  PhantomData,
    };
    println!("{}", bit_invariant(&narrow, carried));
}
