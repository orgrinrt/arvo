// The caller's trait on a type that does not implement the implementor's.
use arvo_bits::Bits;
use arvo_format::Bool;
use bit_tier_guard_sealed::BitAccess;
#[derive(Clone, Copy)]
struct Bare(Bits<8>);
impl BitAccess for Bare {
    fn bit<const I: u32>(self) -> Bool {
        Bool::of(false)
    }

    fn with_bit<const I: u32>(self, _: Bool) -> Self {
        self
    }
}
fn main() {}
