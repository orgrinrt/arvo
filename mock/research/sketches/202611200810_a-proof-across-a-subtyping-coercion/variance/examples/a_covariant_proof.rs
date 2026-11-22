//! A proof made for the 128-bit container, carried to the 8-bit one.
use core::marker::PhantomData;

use a_proof_across_a_subtyping_coercion::{Covariant, Narrow, Wide, bit_covariant};

fn main() {
    let proof: Covariant<Wide, 100> = Covariant::NEW;
    let carried: Covariant<Narrow, 100> = proof;
    let narrow: Narrow = Narrow {
        bytes: &[0xFF],
        kind:  PhantomData,
    };
    println!("{}", bit_covariant(&narrow, carried));
}
