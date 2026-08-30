// Probe 7. A second read of file 66 section 1, which concedes more than it had
// to.
//
// File 66 quotes `unstable-features.md` and says "Every sentence but the last
// is correct and I would not change a word of it", then corrects only the last.
// The penultimate sentence is:
//
//   > A design that verifies a claim exhaustively at, say, eight bits and
//   > relies on it at sixty-four is relying on there being no way for a type to
//   > observe which instantiation it is in and behave differently. Full
//   > `specialization` is exactly such a way, and `TypeId` is another.
//
// That is an enumeration, and it is incomplete against arvo's own shipped code.
// `arvo-strategy/src/container.rs:254-280` projects the container type through
// `Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, S>`: the width N
// selects a tag, the tag selects an impl, the impl selects an associated type,
// and the associated type is the arithmetic container. That IS a type observing
// which instantiation it is in and behaving differently. It is not
// specialization and it is not `TypeId`; it is Pattern C const-tag dispatch,
// it is permitted, and it is load-bearing in a shipped crate.
//
// This probe reproduces that shape without any forbidden feature and exhibits a
// property that is TRUE at the model width and FALSE at the target width, with
// the bans in force and one parametric function throughout.
//
// The consequence is not that the bans are wrong. It is that they close ONE
// coordinate of the index set, and file 66's section 4 vocabulary is what the
// rest need. Container class is a coordinate, and the model must cover one
// width per class rather than one width overall.

#![allow(dead_code)]

// --------------------------------------------------------------------------
// Pattern C, reduced: a const tag selects an impl selects a container.
// No specialization. No TypeId. No generic_const_exprs.
// --------------------------------------------------------------------------
struct Picker;

trait Project<const TAG: usize> {
    type T: Container;
}

trait Container: Copy {
    const NAME: &'static str;
    fn from_u32(v: u32) -> Self;
    fn to_u32(self) -> u32;
    fn double(self) -> Self;
}

impl Container for u8 {
    const NAME: &'static str = "u8";
    fn from_u32(v: u32) -> Self {
        v as u8
    }
    fn to_u32(self) -> u32 {
        self as u32
    }
    fn double(self) -> Self {
        self.wrapping_mul(2)
    }
}
impl Container for u16 {
    const NAME: &'static str = "u16";
    fn from_u32(v: u32) -> Self {
        v as u16
    }
    fn to_u32(self) -> u32 {
        self as u32
    }
    fn double(self) -> Self {
        self.wrapping_mul(2)
    }
}
impl Container for u32 {
    const NAME: &'static str = "u32";
    fn from_u32(v: u32) -> Self {
        v
    }
    fn to_u32(self) -> u32 {
        self
    }
    fn double(self) -> Self {
        self.wrapping_mul(2)
    }
}

impl Project<0> for Picker {
    type T = u8;
}
impl Project<1> for Picker {
    type T = u16;
}
impl Project<2> for Picker {
    type T = u32;
}

/// The shipped `tag_hot_cold(N)` shape: a const fn from the width to a tag.
const fn tag(n: usize) -> usize {
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else {
        2
    }
}

/// ONE parametric function. No instantiation is given a different body. The
/// bans hold completely. What differs is which container the projection picked.
fn doubling_wraps<const TAG: usize>(v: u32) -> bool
where
    Picker: Project<TAG>,
{
    let c = <<Picker as Project<TAG>>::T as Container>::from_u32(v);
    let d = c.double();
    d.to_u32() != v.wrapping_mul(2)
}

fn name<const TAG: usize>() -> &'static str
where
    Picker: Project<TAG>,
{
    <<Picker as Project<TAG>>::T as Container>::NAME
}

fn main() {
    // The property under test, stated once, for a value that fits in 8 bits:
    //
    //   P(N): for v = 200, doubling v inside the width-N container does not
    //         lose information relative to doubling it in the abstract.
    //
    // 200 is representable at every width from 8 up. The property is about the
    // container, not about representability of the operand.
    const V: u32 = 200;

    let at8 = doubling_wraps::<{ tag(8) }>(V);
    let at9 = doubling_wraps::<{ tag(9) }>(V);
    let at17 = doubling_wraps::<{ tag(17) }>(V);

    println!("  width  container  doubling of {V} wraps");
    println!("      8  {:9}  {}", name::<{ tag(8) }>(), at8);
    println!("      9  {:9}  {}", name::<{ tag(9) }>(), at9);
    println!("     17  {:9}  {}", name::<{ tag(17) }>(), at17);

    // THE RESULT. The property's truth value moves with the width, and it moves
    // at the CONTAINER CLASS BOUNDARY rather than smoothly, so a model that
    // samples one width per decade would miss it while a model that samples one
    // width per container class would not.
    assert!(at8, "at 8 bits the u8 container wraps");
    assert!(!at9, "at 9 bits the u16 container does not");
    assert!(!at17, "at 17 bits the u32 container does not");

    // And the boundary is exactly where the projection changes its answer, not
    // anywhere else: 8 and 9 differ, 9 through 16 agree, 17 differs again.
    assert_eq!(name::<{ tag(8) }>(), "u8");
    assert_eq!(name::<{ tag(9) }>(), "u16");
    assert_eq!(name::<{ tag(16) }>(), "u16");
    assert_eq!(name::<{ tag(17) }>(), "u32");
    assert!(!doubling_wraps::<{ tag(16) }>(V));

    // Negative control: the mechanism is not a disguised specialization. There
    // is one body for `doubling_wraps`, and the only thing that varies is the
    // associated type the projection resolves to. Confirm the same body gives
    // the same answer whenever the projection gives the same container.
    assert_eq!(
        doubling_wraps::<{ tag(9) }>(V),
        doubling_wraps::<{ tag(16) }>(V),
        "same container class must give the same answer"
    );

    // Second negative control: the property is not vacuous at the widths where
    // it holds. A value that genuinely overflows u16 wraps there too, so the
    // u16 arm is doing real arithmetic rather than being unreachable.
    assert!(doubling_wraps::<{ tag(16) }>(40_000));

    println!("\n  ALL ASSERTIONS PASSED");
    println!("  => a property TRUE at 8 bits and FALSE at 9, one parametric body,");
    println!("     no specialization, no TypeId, no forbidden feature.");
}
