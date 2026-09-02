// q13. Cold, which is the workload arvo exists for and which no probe in the
// second stretch instantiated.
//
// The question is not "does Cold pick a different container". It is whether the
// container map's CODOMAIN is even the same kind of thing for Cold as for the
// other three. A bitpacked value has no standalone Rust type: nothing has a size
// of five bits. So either Cold is not a container choice at all, or the map has
// a second output.
//
// The acceptance criterion (SETTLED.md:65-71) says the typestate derives "the
// matching container AND numeral representations". Two nouns. This probe treats
// the second one as load-bearing and tests what it has to be:
//
//   Container  the standalone value's type. Cold agrees with Warm here.
//   Stride     the bits one element occupies inside an aggregate. Cold is W;
//              the others are 8*ceil(W/8).
//
// and then checks that the second output is enough to build the aggregate, which
// is the only way to find out whether it is the right second output.
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. Edition 2024.
// Build: rustc +nightly-2026-05-28 --edition 2024 -O q13_cold_packed.rs \
//          --out-dir build && ./build/q13_cold_packed

#![allow(dead_code)]

include!("q07_body_inc.rs");
include!("q12_door_inc.rs");

// The nat has to reach a usize for the aggregate's byte count. This is the one
// place a value escapes the type level, and it is a const, so it is the same
// escape 13's `Nat::V` uses.
pub trait Val {
    const V: usize;
}
impl Val for Z {
    const V: usize = 0;
}
impl<N: Val> Val for O<N> {
    const V: usize = 2 * N::V + 1;
}
impl<N: Val> Val for E<N> {
    const V: usize = 2 * N::V;
}

// A column of COUNT elements, laid out at the stride the strategy derives.
// Nothing here knows which strategy it is serving: it reads Stride and that is
// the whole interface. Swapping Cold for Warm changes the byte count and not a
// line of this code.
pub struct Column<T, const COUNT: usize>(core::marker::PhantomData<T>);

pub trait Layout {
    const STRIDE_BITS: usize;
    const BYTES: usize;
}
impl<T, const COUNT: usize> Layout for Column<T, COUNT>
where
    T: Derived,
    <T as Derived>::Stride: Val,
{
    const STRIDE_BITS: usize = <<T as Derived>::Stride as Val>::V;
    const BYTES: usize = (Self::STRIDE_BITS * COUNT + 7) / 8;
}

type ColdU5 = UFixed<5, 0, Unsigned, Cold>;
type WarmU5 = UFixed<5, 0, Unsigned, Warm>;
type ColdU13_3 = UFixed<13, 3, Unsigned, Cold>;
type WarmU13_3 = UFixed<13, 3, Unsigned, Warm>;
type ColdU200_40 = UFixed<200, 40, Unsigned, Cold>;
type WarmU200_40 = UFixed<200, 40, Unsigned, Warm>;
type ColdU3 = UFixed<3, 0, Unsigned, Cold>;
type WarmU3 = UFixed<3, 0, Unsigned, Warm>;
// the two rows that caught the stride defect: W = 24 rounds UP a rung, so the
// container is u32 and the stride is 32 rather than 24; and Hot's wide arm pads
// to align 16, so a 25-byte payload occupies 32.
type ColdU20_4 = UFixed<20, 4, Unsigned, Cold>;
type WarmU20_4 = UFixed<20, 4, Unsigned, Warm>;
type HotU200_40 = UFixed<200, 40, Unsigned, Hot>;

const N: usize = 1_000_000;

fn row<T>(name: &str)
where
    Column<T, N>: Layout,
{
    println!(
        "{:<28} stride={:>4} bits   {} elements = {:>9} bytes",
        name,
        <Column<T, N> as Layout>::STRIDE_BITS,
        N,
        <Column<T, N> as Layout>::BYTES
    );
}

// a static assertion that the two strategies really do differ, so the printout
// is a report rather than the evidence
fn _static()
where
    ColdU5: Derived<Stride = N5>,
    WarmU5: Derived<Stride = N8>,
    ColdU3: Derived<Stride = N3>,
    WarmU3: Derived<Stride = N8>,
    ColdU200_40: Derived<Stride = N240>,
    WarmU200_40: Derived<Stride = N240>,
    // and the container is the SAME for both, which is the point: Cold is not a
    // different container, it is a different aggregate representation
    ColdU13_3: Derived<Container = u16>,
    WarmU13_3: Derived<Container = u16>,
    // W = 24: the container is u32 and the stride follows the container
    ColdU20_4: Derived<Container = u32, Stride = N24>,
    WarmU20_4: Derived<Container = u32, Stride = N32>,
    // the wide rung: Hot pads to align 16 and its stride says so
    WarmU200_40: Derived<Stride = N240>,
    HotU200_40: Derived<Stride = N256>,
{
}

fn main() {
    println!("one million elements, per strategy and width:\n");
    row::<ColdU3>("UFixed<3,0,U,Cold>");
    row::<WarmU3>("UFixed<3,0,U,Warm>");
    row::<ColdU5>("UFixed<5,0,U,Cold>");
    row::<WarmU5>("UFixed<5,0,U,Warm>");
    row::<ColdU13_3>("UFixed<13,3,U,Cold>");
    row::<WarmU13_3>("UFixed<13,3,U,Warm>");
    row::<ColdU20_4>("UFixed<20,4,U,Cold>");
    row::<WarmU20_4>("UFixed<20,4,U,Warm>");
    row::<ColdU200_40>("UFixed<200,40,U,Cold>");
    row::<WarmU200_40>("UFixed<200,40,U,Warm>");
    row::<HotU200_40>("UFixed<200,40,U,Hot>");
    println!();
    println!(
        "the standalone container is the same for Cold and Warm at 13.3: {} vs {} bytes",
        core::mem::size_of::<<ColdU13_3 as Derived>::Container>(),
        core::mem::size_of::<<WarmU13_3 as Derived>::Container>()
    );
    println!();
    println!("the derived stride against the container's real size, in bits:");
    macro_rules! chk {
        ($n:expr, $t:ty) => {
            println!(
                "  {:<24} derived={:>4}   8*size_of(container)={:>4}",
                $n,
                <<$t as Derived>::Stride as Val>::V,
                8 * core::mem::size_of::<<$t as Derived>::Container>()
            );
        };
    }
    chk!("UFixed<20,4,U,Warm>", WarmU20_4);
    chk!("UFixed<20,4,U,Cold>", ColdU20_4);
    chk!("UFixed<200,40,U,Warm>", WarmU200_40);
    chk!("UFixed<200,40,U,Hot>", HotU200_40);
    chk!("UFixed<200,40,U,Cold>", ColdU200_40);
    chk!("UFixed<5,0,U,Warm>", WarmU5);
    chk!("UFixed<5,0,U,Cold>", ColdU5);
}
