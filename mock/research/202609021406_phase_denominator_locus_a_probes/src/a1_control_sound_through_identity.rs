// A1, the positive control. The sound declaration through the one path that
// forces `Format::ADMITTED`. This must build, or every refusal below is a
// property of the harness rather than of the denominator.
mod shared;
use arvo_format::format::has_additive_identity;

fn main() {
    println!(
        "A1 sound/has_additive_identity = {}",
        has_additive_identity::<shared::Sound>().get()
    );
}
