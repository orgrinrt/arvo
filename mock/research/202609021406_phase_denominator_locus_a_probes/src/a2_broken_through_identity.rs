// A2, the case that must fail. The offending declaration through the same path.
mod shared;
use arvo_format::format::has_additive_identity;

fn main() {
    println!(
        "A2 broken/has_additive_identity = {}",
        has_additive_identity::<shared::Broken>().get()
    );
}
