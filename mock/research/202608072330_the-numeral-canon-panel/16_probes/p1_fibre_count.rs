// p1: how badly does the carrier alone lose information?
//
// Checks ONE thing: the map (width, sign, strategy) -> carrier is not injective, and by how much.
// If it is not injective, a derivation whose whole codomain is the carrier has destroyed
// distinctions at the moment it returns, and no downstream site can recover them.
//
// Every number this prints is computed here. No number is quoted from anywhere.
//
//   rustc +nightly-2026-05-28 -O p1_fibre_count.rs -o /tmp/p1 && /tmp/p1
//
// Spike. Its names and shapes are scaffolding, not proposals.

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Strat {
    Hot,
    Warm,
    Cold,
    Precise,
}

const STRATS: [Strat; 4] = [Strat::Hot, Strat::Warm, Strat::Cold, Strat::Precise];

// A carrier is identified by (bits, signed). This is the ONE-OUTPUT derivation:
// the whole answer is a machine type.
const fn carrier_bits(w: u32) -> u32 {
    // smallest native container that holds w bits
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else if w <= 64 {
        64
    } else if w <= 128 {
        128
    } else {
        // wide rung: whole bytes, rounded up, no ceiling
        w.div_ceil(8) * 8
    }
}

// The SECOND output. Stride is the bit distance between consecutive elements when N of them
// are stored contiguously. Only Cold packs; the other three pay the carrier's full width.
const fn stride_bits(w: u32, s: Strat) -> u32 {
    match s {
        Strat::Cold => w,
        _ => carrier_bits(w),
    }
}

fn main() {
    const MAXW: u32 = 128;

    // 1. Domain and codomain sizes.
    let mut declarations = 0usize;
    let mut carriers: Vec<(u32, bool)> = Vec::new();
    let mut layouts: Vec<(u32, bool, u32)> = Vec::new(); // (carrier bits, signed, stride)

    for w in 1..=MAXW {
        for signed in [false, true] {
            for s in STRATS {
                declarations += 1;
                let c = (carrier_bits(w), signed);
                if !carriers.contains(&c) {
                    carriers.push(c);
                }
                let l = (carrier_bits(w), signed, stride_bits(w, s));
                if !layouts.contains(&l) {
                    layouts.push(l);
                }
            }
        }
    }

    println!("widths 1..={MAXW}, 2 signs, 4 strategies");
    println!("  distinct declarations      : {declarations}");
    println!("  distinct carriers          : {}", carriers.len());
    println!("  distinct (carrier, stride) : {}", layouts.len());
    println!(
        "  declarations per carrier   : {:.1} average",
        declarations as f64 / carriers.len() as f64
    );

    // 2. The fibre that matters: unsigned Cold values sharing one carrier but needing
    //    different layouts. This is the collapse, exhibited.
    println!();
    println!("unsigned Cold, grouped by carrier; every row inside a group shares a carrier");
    for cb in [8u32, 16, 32, 64, 128] {
        let widths: Vec<u32> = (1..=MAXW).filter(|&w| carrier_bits(w) == cb).collect();
        let strides: Vec<u32> = widths
            .iter()
            .map(|&w| stride_bits(w, Strat::Cold))
            .collect();
        println!(
            "  carrier u{cb:<3} <- {} widths, strides {}..={} ({} distinct layouts collapsed to 1)",
            widths.len(),
            strides.iter().min().unwrap(),
            strides.iter().max().unwrap(),
            strides.len()
        );
    }

    // 3. The cost of the collapse, as bytes, for a contiguous run.
    //    Sizes only. Nothing here is a timing and nothing here is priced.
    println!();
    println!("contiguous run of N = 1_000_000 values, unsigned Cold");
    println!("  W    carrier-only bytes   two-output bytes   carrier-only overhead");
    const N: u64 = 1_000_000;
    for w in [3u32, 5, 9, 11, 13, 17, 23, 27, 31, 47, 63] {
        let carrier_only = (carrier_bits(w) as u64 / 8) * N;
        let two_output = (stride_bits(w, Strat::Cold) as u64 * N).div_ceil(8);
        println!(
            "  {w:<4} {carrier_only:>18}   {two_output:>16}   {:>6.1}%",
            (carrier_only as f64 / two_output as f64 - 1.0) * 100.0
        );
    }

    // 4. The worst case in the range, found rather than assumed.
    let mut worst_w = 0u32;
    let mut worst_ratio = 0.0f64;
    for w in 1..=MAXW {
        let r = carrier_bits(w) as f64 / w as f64;
        if r > worst_ratio {
            worst_ratio = r;
            worst_w = w;
        }
    }
    println!();
    println!("worst ratio in 1..={MAXW}: W = {worst_w}, carrier-only is {worst_ratio:.2}x the packed size");
}
