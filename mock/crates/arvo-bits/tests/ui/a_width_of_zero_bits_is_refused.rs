// A declared width of zero bits admits no values and is not a bit container.
// `ADMITTED` is forced inside `masked`, so the call is what reaches the
// assertion; nothing about `N` itself is checked until something uses it.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<0>::masked(0);
};

fn main() {}
