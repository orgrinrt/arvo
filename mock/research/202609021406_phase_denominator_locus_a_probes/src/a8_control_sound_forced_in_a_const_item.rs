// A8, the control for A7. The sound declaration in the identical const item.
mod shared;
use arvo_format::format::has_additive_identity;
use arvo_format::width::Bool;

const ACCEPTED: Bool = has_additive_identity::<shared::Sound>();

fn main() {
    println!("A8 sound in a const item = {}", ACCEPTED.get());
}
