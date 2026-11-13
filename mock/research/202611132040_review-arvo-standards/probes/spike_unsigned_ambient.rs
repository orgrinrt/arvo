use arvo_format::ambient::UnsignedBinaryRationals;
use arvo_format::format::{contains, is_admissible_format};
use arvo_format::quantum::Magnitude;
use arvo_format::slots::Slot;
use arvo_format::standards::ieee754::{Binary, Interchange};
type U = Interchange<UnsignedBinaryRationals, 24, 127>;
type Z = Binary<0, 1>;
#[test]
fn unsigned_ambient_with_negative_members() {
    let neg = contains::<U>(Slot::at(-1), Magnitude::SMALLEST).get();
    let adm = is_admissible_format::<U>().get();
    let z = contains::<Z>(Slot::at(0), Magnitude::SMALLEST).get();
    panic!(
        "negative member in unsigned ambient: {neg}, admissible: {adm}, p=0 format admits zero: {z}"
    );
}
