// Control. No gate: the refusal `tests/ui/an_arvo_type_as_a_const_parameter.rs`
// already pins, reproduced standalone so the arms below are compared against it
// rather than against a memory of it.
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Width(u32);

pub struct Signed<const BITS: Width>;

fn main() {}
