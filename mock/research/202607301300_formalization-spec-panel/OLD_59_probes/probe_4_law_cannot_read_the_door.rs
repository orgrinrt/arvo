//! Probe 4. "No law may read `Lowering`" (58:163-169) is enforced by the
//! language, not by discipline, once a law trait is keyed on the numeral alone.
//!
//! This matters more now than before: probe_1 gives the door a const face
//! (`DeclaresEnv::DECLARED`) so a build layer can read the receipt off the
//! type. A const face is exactly the shape that could leak into a law's key.
//! The claim under test is that it cannot, and that the refusal is mechanical.
//!
//! NEGATIVE CONTROL. Expected: E0207, twice.

pub trait Numeral: 'static {}
pub struct Binary32;
impl Numeral for Binary32 {}

pub trait LoweringDoor {}
pub struct Quantised;
pub struct HostFloat;
impl LoweringDoor for Quantised {}
impl LoweringDoor for HostFloat {}

pub trait Lowering {
    type Door: LoweringDoor;
    const HAZARDOUS_FACE: bool;
}
pub struct Soft;
pub struct Hard;
impl Lowering for Soft {
    type Door = Quantised;
    const HAZARDOUS_FACE: bool = false;
}
impl Lowering for Hard {
    type Door = HostFloat;
    const HAZARDOUS_FACE: bool = true;
}

/// A law. Keyed on the numeral and nothing else, which is the whole of the
/// ratified statement: the key is a `const fn` parameter list and `Lowering`
/// is not a parameter.
pub trait AddCommutes<N: Numeral> {}

pub struct Witness;

/// Attempt one: hold the law only where the lowering is the software door.
/// `L` appears nowhere in the trait or the self type.
impl<N: Numeral, L: Lowering<Door = Quantised>> AddCommutes<N> for Witness {}

/// Attempt two: hold it only where the receipt's own const face says so. Same
/// wall, and it is the const face specifically that this one tries to read.
impl<N: Numeral, L: Lowering> AddCommutes<N> for (Witness, [(); L::HAZARDOUS_FACE as usize]) {}

fn main() {}
