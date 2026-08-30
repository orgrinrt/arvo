// d2: does #[diagnostic::on_unimplemented] substitute a CONST generic parameter's
// value into the message, and a TYPE parameter's name?
#![no_std]

pub struct Idx<const N: u32>;

#[diagnostic::on_unimplemented(
    message = "no fixed-point numeral of width {N} exists",
    label = "width {N} is not admitted",
    note = "Self is `{Self}`, the type parameter T is `{T}`"
)]
pub trait ToNat<T> {
    type N;
}

pub struct Term;
pub struct Marker;
impl ToNat<Marker> for Idx<13> {
    type N = Term;
}

pub fn ok(_: <Idx<13> as ToNat<Marker>>::N) {}
pub fn bad(_: <Idx<14> as ToNat<Marker>>::N) {}
