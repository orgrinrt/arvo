// P1. The constructor is total and lossless on the offending pair, which is what
// the source claims and what every arm above rests on.
use arvo_format::format::Phase;

fn main() {
    const BROKEN: Phase = Phase::of(1, 0);
    const SOUND: Phase = Phase::of(1, 2);
    println!("P1 broken = {}/{} denotes={}", BROKEN.numerator(), BROKEN.denominator(), BROKEN.denotes().get());
    println!("P1 sound  = {}/{} denotes={}", SOUND.numerator(), SOUND.denominator(), SOUND.denotes().get());
    assert_eq!(BROKEN.denominator(), 0, "the pair came back changed");
    assert!(!BROKEN.denotes().get(), "the offending pair reports that it denotes");
    assert!(SOUND.denotes().get(), "the control reports that it does not denote");
}
