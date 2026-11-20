// A list of fields of a consumer's own, claiming a sum of thirty-two bits with no
// field behind it, is refused: `Fields` is sealed.

use arvo_bits::Fields;
use arvo_format::Width;

struct Liar;

impl Fields for Liar {
    const SUM: Width = Width::bits(32);
}

// No layout names `Liar`: its `Fields` bound would be refused as well, and rustc
// has put the two refusals in either order from one build to the next.
fn main() {}
