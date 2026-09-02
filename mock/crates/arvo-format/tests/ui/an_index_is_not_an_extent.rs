// A magnitude where a count of magnitudes is wanted, and the reverse.
//
// The off-by-one both types exist for. While the two were one `u32` this was
// well-typed and silently wrong, and nothing in the suite could name it.

use arvo_format::quantum::{Magnitude, MagnitudeCount};

fn main() {
    let count = MagnitudeCount::of(30);
    let index = Magnitude::at(29);

    // A count is not an index into itself.
    let _ = index.is_within(index);

    // And an index is not the extent it sits inside.
    let _: MagnitudeCount = index;
    let _: Magnitude = count;
}
