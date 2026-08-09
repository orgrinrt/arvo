// TEST B: the coherence budget of "absence of impl = mathematical falsity".
// The spec has two impls keyed on Signedness. Add ONE law that holds
// uniformly in signedness for a particular resolution (wrapping is a group
// either way) and the encoding hits overlap with no escape, since full
// specialization is forbidden and min_specialization cannot order these.

pub trait Signedness {}
pub struct Unsigned;
impl Signedness for Unsigned {}
pub struct Signed;
impl Signedness for Signed {}

pub trait Resolution {}
pub struct TowardNegative;
impl Resolution for TowardNegative {}
pub struct ReduceModulo;
impl Resolution for ReduceModulo {}

pub trait Faithful: Resolution {}
impl Faithful for ReduceModulo {}

pub trait AddAssoc {}

// the spec's two impls
impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}
impl<A: Faithful, B: Faithful> AddAssoc for ((A, B), Signed) {}

// a third, signedness-uniform, fact: wrapping folds whatever the sign is.
// Mathematically true and one anyone would want to state once.
impl<S: Signedness> AddAssoc for ((ReduceModulo, ReduceModulo), S) {}

fn main() {}
