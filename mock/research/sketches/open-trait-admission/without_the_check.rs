pub trait Slots {
    const MIN: i64;
    const MAX: i64;
    const WIDTH: u32;
    const ADMITTED: () = {
        assert!(
            Self::MIN <= Self::MAX,
            "slot range is inverted: MIN exceeds MAX"
        );
        assert!(
            Self::WIDTH >= 1,
            "a declared width of zero admits no values"
        );
        assert!(
            Self::WIDTH <= 62,
            "declared width is wider than a slot index carries"
        );
    };
}
pub const fn slot_count<S: Slots>() -> i64 {
    S::MAX - S::MIN + 1
}

// The reviewer's construction, values verbatim.
struct Rogue;
impl Slots for Rogue {
    const MIN: i64 = 4611686018427387904;
    const MAX: i64 = -4611686018427387905;
    const WIDTH: u32 = 63;
}

fn main() {
    println!("{}", slot_count::<Rogue>());
}
