// p5: if Precise widens compute past storage, is a THIRD output (a compute carrier distinct
// from the storage carrier) mechanically available, gate-free, using the same trait-based
// derivation `16`'s p6 already builds for two?
//
// This checks the OTHER half of the dispatch question. Whatever the Precise question
// decides, the type-system side is not what is blocking it: `a-refused-bound-wants-a-
// trait-not-a-feature.md` names the general move (decompose the refused const-expression
// into named trait associated items, computed in an impl rather than in a bound), and it
// already worked once for (Carrier, Stride) in 16_probes/p6. This probe extends the SAME
// trait with a third associated item, `ComputeCarrier`, which equals `Carrier` for every
// strategy that does not widen and diverges only for a strategy that does, and checks that
// the extension costs nothing structurally: same arity across all four strategies (per
// 16's own principle, "the arity does not change with the strategy"), no new feature gate,
// and the two readings of Precise (widens / does not widen) are BOTH expressible in this
// shape without touching the mechanism, only the one impl block for Precise.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p5_third_output_is_mechanically_free.rs -o bin/p5 && ./bin/p5
//   rustc +nightly-2026-05-28 --edition 2021 --cfg precise_widens -O p5_third_output_is_mechanically_free.rs -o bin/p5_widens && ./bin/p5_widens
//
// Spike.

trait Rung {
    const BITS: u32;
}
struct R8;
struct R16;
struct R32;
struct R64;
struct R128;
impl Rung for R8 {
    const BITS: u32 = 8;
}
impl Rung for R16 {
    const BITS: u32 = 16;
}
impl Rung for R32 {
    const BITS: u32 = 32;
}
impl Rung for R64 {
    const BITS: u32 = 64;
}
impl Rung for R128 {
    const BITS: u32 = 128;
}

// Strategy markers, matching arvo's own four.
struct Hot;
struct Warm;
struct Cold;
struct Precise;

// Three associated items per (strategy, rung): the storage carrier's bit width, the stride,
// and the compute carrier's bit width. All three are const u32 here (a stand-in for a real
// carrier TYPE, since the point is the arity and the mechanism, not the concrete container).
trait Derive3<R: Rung> {
    const CARRIER_BITS: u32;
    const STRIDE_BITS: u32;
    const COMPUTE_CARRIER_BITS: u32;
}

macro_rules! impl_non_widening {
    ($strat:ty, $rung:ty) => {
        impl Derive3<$rung> for $strat {
            const CARRIER_BITS: u32 = <$rung as Rung>::BITS;
            const STRIDE_BITS: u32 = <$rung as Rung>::BITS;
            // does not widen: compute carrier equals the storage carrier, for every
            // strategy that does not diverge compute from storage.
            const COMPUTE_CARRIER_BITS: u32 = <$rung as Rung>::BITS;
        }
    };
}

impl_non_widening!(Hot, R8);
impl_non_widening!(Hot, R16);
impl_non_widening!(Hot, R32);
impl_non_widening!(Warm, R8);
impl_non_widening!(Warm, R16);
impl_non_widening!(Warm, R32);
impl_non_widening!(Cold, R8);
impl_non_widening!(Cold, R16);
impl_non_widening!(Cold, R32);

// Cold's stride is not the rung's width; it is the declared width, per 15/16. Modelled here
// with a separate, narrower rung to keep the macro simple: Cold at the packed width uses its
// OWN impl rather than the shared macro, matching 15/16's finding that Cold's stride and
// carrier are genuinely different quantities. Left out of the macro above deliberately; see
// the standalone impl below for one packed case, enough to show the mechanism is unaffected.
struct ColdPacked13;
impl Rung for ColdPacked13 {
    const BITS: u32 = 13;
}
impl Derive3<ColdPacked13> for Cold {
    const CARRIER_BITS: u32 = 16; // the standalone value's carrier is the native rung
    const STRIDE_BITS: u32 = 13; // the packed stride is the declared width
    const COMPUTE_CARRIER_BITS: u32 = 16; // Cold does not widen compute
}

// Precise, reading A: does not widen. Same shape as Hot/Warm/Cold.
#[cfg(not(precise_widens))]
impl_non_widening!(Precise, R8);
#[cfg(not(precise_widens))]
impl_non_widening!(Precise, R16);
#[cfg(not(precise_widens))]
impl_non_widening!(Precise, R32);

// Precise, reading B: widens compute to the next native rung up, storage unchanged. Only
// this one impl block differs from reading A; nothing else in the mechanism moves.
#[cfg(precise_widens)]
impl Derive3<R8> for Precise {
    const CARRIER_BITS: u32 = 8;
    const STRIDE_BITS: u32 = 8;
    const COMPUTE_CARRIER_BITS: u32 = 16; // widened
}
#[cfg(precise_widens)]
impl Derive3<R16> for Precise {
    const CARRIER_BITS: u32 = 16;
    const STRIDE_BITS: u32 = 16;
    const COMPUTE_CARRIER_BITS: u32 = 32; // widened
}
#[cfg(precise_widens)]
impl Derive3<R32> for Precise {
    const CARRIER_BITS: u32 = 32;
    const STRIDE_BITS: u32 = 32;
    const COMPUTE_CARRIER_BITS: u32 = 64; // widened
}

fn row<S, R: Rung>(name: &str, rung_bits: u32)
where
    S: Derive3<R>,
{
    println!(
        "{name:<10} rung={rung_bits:>4}  carrier={:>4}  stride={:>4}  compute_carrier={:>4}  {}",
        S::CARRIER_BITS,
        S::STRIDE_BITS,
        S::COMPUTE_CARRIER_BITS,
        if S::COMPUTE_CARRIER_BITS != S::CARRIER_BITS {
            "<- compute diverges from storage"
        } else {
            ""
        }
    );
}

fn main() {
    println!("cfg(precise_widens) = {}", cfg!(precise_widens));
    println!();
    row::<Hot, R16>("Hot", 16);
    row::<Warm, R16>("Warm", 16);
    row::<Cold, R16>("Cold", 16);
    row::<Cold, ColdPacked13>("Cold", 13);
    row::<Precise, R16>("Precise", 16);
    println!();
    println!("three associated items on one trait, same arity for every strategy at every");
    println!("rung, zero feature gates. switching between the two readings of Precise is a");
    println!("single impl block; nothing else in the mechanism, and nothing in the other");
    println!("three strategies' impls, needs to move either way.");
}
