//! Probe 6. What door a mixed-strategy expression takes. The presumptive
//! per-preset table (58:820-822) assigns a door per preset and says nothing
//! about an expression whose operands disagree, which every real consumer
//! surface has.
//!
//! Nothing new is needed. arvo already resolves cross-strategy operations by
//! `Strategy::RANK`, "higher is more conservative", `Precise > Cold > Warm >
//! Hot` (arvo-strategy/src/lib.rs:105-108). `Hot` is the LOWEST rank and the
//! only preset whose default door is the hardware one, so the resolution
//! carries a theorem for free.
//!
//! POSITIVE, and the theorem is the point:
//!
//!   THE HARDWARE DOOR IS REACHABLE ONLY IN A UNIFORMLY-`Hot` EXPRESSION.
//!
//! A mixed expression cannot silently acquire hardware semantics, because the
//! resolution moves away from `Hot` by construction and every other preset's
//! door is the quantiser. That is not a rule anyone has to write down or
//! enforce; it is a consequence of a rank ordering that shipped years before
//! the door question was asked.

pub trait Strategy: 'static {
    const RANK: u16;
    const NAME: &'static str;
}
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;
macro_rules! s {
    ($t:ty, $r:expr, $n:expr) => {
        impl Strategy for $t {
            const RANK: u16 = $r;
            const NAME: &'static str = $n;
        }
    };
}
s!(Hot, 0, "Hot");
s!(Warm, 1, "Warm");
s!(Cold, 2, "Cold");
s!(Precise, 3, "Precise");

/// Cross-strategy resolution, the shipped rule: the more conservative side
/// wins. Modelled as a projection rather than as a const comparison, because a
/// const comparison over two generic strategies is the `generic_const_exprs`
/// wall again (probe_5).
pub trait Resolve<Rhs: Strategy>: Strategy {
    type Out: Strategy;
}
macro_rules! r {
    ($a:ty, $b:ty, $o:ty) => {
        impl Resolve<$b> for $a {
            type Out = $o;
        }
    };
}
r!(Hot, Hot, Hot);
r!(Hot, Warm, Warm);
r!(Hot, Cold, Cold);
r!(Hot, Precise, Precise);
r!(Warm, Hot, Warm);
r!(Warm, Warm, Warm);
r!(Warm, Cold, Cold);
r!(Warm, Precise, Precise);
r!(Cold, Hot, Cold);
r!(Cold, Warm, Cold);
r!(Cold, Cold, Cold);
r!(Cold, Precise, Precise);
r!(Precise, Hot, Precise);
r!(Precise, Warm, Precise);
r!(Precise, Cold, Precise);
r!(Precise, Precise, Precise);

pub trait LoweringDoor {
    const IS_HARDWARE: bool;
}
pub struct Quantised;
pub struct HostFloat;
impl LoweringDoor for Quantised {
    const IS_HARDWARE: bool = false;
}
impl LoweringDoor for HostFloat {
    const IS_HARDWARE: bool = true;
}

/// The per-preset default from probe_3c, over a host-implemented numeral.
pub trait DefaultDoor: Strategy {
    type D: LoweringDoor;
}
impl DefaultDoor for Hot {
    type D = HostFloat;
}
impl DefaultDoor for Warm {
    type D = Quantised;
}
impl DefaultDoor for Cold {
    type D = Quantised;
}
impl DefaultDoor for Precise {
    type D = Quantised;
}

fn door<A: Resolve<B>, B: Strategy>() -> (&'static str, bool)
where
    <A as Resolve<B>>::Out: DefaultDoor,
{
    (
        <<A as Resolve<B>>::Out as Strategy>::NAME,
        <<<A as Resolve<B>>::Out as DefaultDoor>::D as LoweringDoor>::IS_HARDWARE,
    )
}

fn main() {
    // The whole 4x4 matrix. The hardware door appears in exactly one cell.
    let mut hardware_cells = Vec::new();
    macro_rules! cell {
        ($a:ty, $b:ty) => {{
            let (name, hw) = door::<$a, $b>();
            if hw {
                hardware_cells.push((stringify!($a), stringify!($b), name));
            }
        }};
    }
    cell!(Hot, Hot);
    cell!(Hot, Warm);
    cell!(Hot, Cold);
    cell!(Hot, Precise);
    cell!(Warm, Hot);
    cell!(Warm, Warm);
    cell!(Warm, Cold);
    cell!(Warm, Precise);
    cell!(Cold, Hot);
    cell!(Cold, Warm);
    cell!(Cold, Cold);
    cell!(Cold, Precise);
    cell!(Precise, Hot);
    cell!(Precise, Warm);
    cell!(Precise, Cold);
    cell!(Precise, Precise);
    assert_eq!(
        hardware_cells,
        vec![("Hot", "Hot", "Hot")],
        "the hardware door must be reachable only from a uniformly-Hot expression"
    );
    println!("probe_6 WORKS: 1 of 16 cells reaches the hardware door, and it is (Hot, Hot)");
}
