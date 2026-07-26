fn main() {
    // arvo_fast_math is set by hilavitkutin-build's FastMath pragma.
    // Declare the cfg so rustc does not warn on #[cfg(arvo_fast_math)].
    //
    // This is the only build script arvo has. It has nothing to do with the
    // design tooling, which the `cargo mock` launcher runs from outside the
    // build: a build-dependency on it would ship in the published manifest
    // and make every consumer fetch it for a tool that does nothing at their
    // build time.
    println!("cargo::rustc-check-cfg=cfg(arvo_fast_math)");
}
