//! P2c. The footprints, printed rather than asserted. Companion to
//! `fibre_refuted.rs`, whose compile error is the qualitative finding.
#[repr(transparent)]
#[derive(Clone, Copy)]
struct Dense13(u16);
#[repr(transparent)]
#[derive(Clone, Copy)]
struct Packed13Elem([bool; 13]);
#[repr(transparent)]
#[derive(Clone, Copy)]
struct BitfieldNewtype(u16);

const W: usize = 13;
const N: usize = 33_554_432; // MAX_N, bitpack-footprint-shared/src/lib.rs:101

fn main() {
    let d = core::mem::size_of::<Dense13>() * 8;
    let p = core::mem::size_of::<Packed13Elem>() * 8;
    let b = core::mem::size_of::<BitfieldNewtype>() * 8;
    println!("logical width                     : {W} bits");
    println!("Dense13 (u16 newtype)             : {d} bits per value");
    println!("BitfieldNewtype (u16)             : {b} bits per value");
    println!("Packed13Elem ([bool; 13])         : {p} bits per value");
    println!("packed-in-column, amortised       : {W} bits per value (no standalone size)");
    println!();
    println!("column of N = {N} elements:");
    println!("  dense  : {} bytes", N * 2);
    println!("  packed : {} bytes", (N * W) / 8);
    println!("  ratio  : {:.4}x", (N * 2) as f64 / ((N * W) / 8) as f64);
    println!();
    println!(
        "smallest expressible standalone value / logical width = {}x",
        p as f64 / W as f64
    );
}
