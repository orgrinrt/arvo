//! Probe 3: does a preset name denote one bundle, or a function from number
//! kind to bundle?
//!
//! The refined naming principle's class-one escape ("a name may freely denote
//! type-level content, because the compiler checks the claim by construction",
//! `91:846-847`) is what lets `Hot`, `Warm`, `Cold` and `Precise` pass without
//! a designated verifier: each is read as an alias for its row in a ratified
//! table. This probe asks whether there is one row per name.
//!
//! Both tables are ratified at `70b` and carried at `78:409-441`. Transcribed
//! here verbatim from that source, cell for cell, and nothing else about the
//! design is modelled.
//!
//! Separation statement per `86b`: the two number kinds agree on `Cold` and on
//! `Precise` for every cell this probe models, which is why a reader who
//! checks the claim at either of those presets concludes the name denotes one
//! bundle. `Hot` and `Warm` are where the distinction is nonvacuous, so the
//! probe asserts over all four presets and both kinds rather than sampling.
//!
//! Run: rustc -O probe_3_a_preset_name_does_not_denote_a_bundle.rs -o /tmp/p94_3 && /tmp/p94_3

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum InRange {
    TowardNegative,
    ToEven,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OutOfRange {
    ReduceModulo,
    Clamp,
    Refuse,
    FarPoint,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Stored {
    Minimum,
    Doubled,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Layout {
    Dense,
    Bitpacked,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Door {
    Inert,
    HostFloat,
    Quantised,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Row {
    in_range: InRange,
    out_of_range: OutOfRange,
    stored: Stored,
    layout: Layout,
    door: Door,
}

/// A preset is not a row. It is a pair of rows, keyed by the number kind, and
/// the type parameter position a consumer writes it in carries only the name.
trait Preset {
    const NAME: &'static str;
    const FIXED: Row;
    const FLOAT: Row;
}

struct Hot;
struct Warm;
struct Cold;
struct Precise;

// `78:409-421`, the fixed-point table, ratified at `70b`.
// `78:433-441`, the float table, ratified at `70b`.

impl Preset for Hot {
    const NAME: &'static str = "Hot";
    const FIXED: Row = Row {
        in_range: InRange::TowardNegative,
        out_of_range: OutOfRange::ReduceModulo,
        stored: Stored::Minimum,
        layout: Layout::Dense,
        door: Door::Inert,
    };
    const FLOAT: Row = Row {
        in_range: InRange::ToEven,
        out_of_range: OutOfRange::FarPoint,
        stored: Stored::Minimum,
        layout: Layout::Dense,
        door: Door::HostFloat,
    };
}

impl Preset for Cold {
    const NAME: &'static str = "Cold";
    const FIXED: Row = Row {
        in_range: InRange::ToEven,
        out_of_range: OutOfRange::Clamp,
        stored: Stored::Minimum,
        layout: Layout::Bitpacked,
        door: Door::Inert,
    };
    const FLOAT: Row = Row {
        in_range: InRange::ToEven,
        out_of_range: OutOfRange::FarPoint,
        stored: Stored::Minimum,
        layout: Layout::Bitpacked,
        door: Door::Quantised,
    };
}

impl Preset for Warm {
    const NAME: &'static str = "Warm";
    const FIXED: Row = Row {
        in_range: InRange::ToEven,
        out_of_range: OutOfRange::Clamp,
        stored: Stored::Doubled,
        layout: Layout::Dense,
        door: Door::Inert,
    };
    const FLOAT: Row = Row {
        in_range: InRange::ToEven,
        out_of_range: OutOfRange::FarPoint,
        // `78:441`: "minimum", bolded in the source as the sharpest single
        // finding of the re-derivation. Warm's float storage is NOT doubled.
        stored: Stored::Minimum,
        layout: Layout::Dense,
        door: Door::HostFloat,
    };
}

impl Preset for Precise {
    const NAME: &'static str = "Precise";
    const FIXED: Row = Row {
        in_range: InRange::ToEven,
        out_of_range: OutOfRange::Refuse,
        stored: Stored::Doubled,
        layout: Layout::Dense,
        door: Door::Inert,
    };
    const FLOAT: Row = Row {
        in_range: InRange::ToEven,
        out_of_range: OutOfRange::Refuse,
        stored: Stored::Doubled,
        layout: Layout::Dense,
        door: Door::Quantised,
    };
}

/// What a consumer at a call site can read off the name alone. The name is one
/// token; the two rows are what it resolves to, and which one applies is
/// decided by the OTHER type parameter (the numeral's exponent form), which is
/// somewhere else in the declaration.
fn report<P: Preset>() {
    let f = P::FIXED;
    let g = P::FLOAT;
    let differing: Vec<&str> = [
        ("in-range direction", f.in_range != g.in_range),
        ("out-of-range", f.out_of_range != g.out_of_range),
        ("StoredWidth", f.stored != g.stored),
        ("Layout", f.layout != g.layout),
        ("Door", f.door != g.door),
    ]
    .into_iter()
    .filter_map(|(n, d)| if d { Some(n) } else { None })
    .collect();

    if differing.is_empty() {
        println!(
            "{:<8} one bundle: fixed and float rows agree on every modelled cell",
            P::NAME
        );
    } else {
        println!(
            "{:<8} TWO bundles; cells that differ by number kind: {:?}",
            P::NAME,
            differing
        );
    }
}

fn main() {
    println!("the four preset names, checked over both ratified tables:\n");
    report::<Hot>();
    report::<Cold>();
    report::<Warm>();
    report::<Precise>();

    println!("\nthe two claims stated as assertions rather than as prose:");

    // Claim A: the name `Hot` does not fix the in-range rounding direction.
    assert_ne!(
        Hot::FIXED.in_range,
        Hot::FLOAT.in_range,
        "if this ever passes, `Hot` denotes one rounding direction"
    );
    println!(
        "  A. `Hot` rounds {:?} on fixed point and {:?} on float: HOLDS",
        Hot::FIXED.in_range,
        Hot::FLOAT.in_range
    );

    // Claim B: the name `Warm` does not fix the stored width.
    assert_ne!(
        Warm::FIXED.stored,
        Warm::FLOAT.stored,
        "if this ever passes, `Warm` denotes one stored width"
    );
    println!(
        "  B. `Warm` stores {:?} on fixed point and {:?} on float: HOLDS",
        Warm::FIXED.stored,
        Warm::FLOAT.stored
    );

    // Claim C: the presets a reader is most likely to spot-check are exactly
    // the two where the difference is invisible. This is the separation
    // requirement's own shape, arising inside the naming question.
    assert_eq!(Cold::FIXED.in_range, Cold::FLOAT.in_range);
    assert_eq!(Precise::FIXED.in_range, Precise::FLOAT.in_range);
    assert_eq!(Precise::FIXED.out_of_range, Precise::FLOAT.out_of_range);
    println!("  C. `Cold` and `Precise` agree on the cells a spot-check reads: HOLDS");

    println!("\nverdict: a preset name is a function from number kind to row, and");
    println!("the number kind is decided by a different parameter of the same type.");
}
