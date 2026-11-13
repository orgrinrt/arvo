//! Must not build: the carrier projected through `Sole`.
use an_unnameable_carrier::Sole;

fn carrier<S: Sole>(x: S::Repr) -> S::Repr {
    x
}

fn main() {}
