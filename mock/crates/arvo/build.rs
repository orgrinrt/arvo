fn main() {
    // arvo_fast_math is set by hilavitkutin-build's FastMath pragma.
    // Declare the cfg so rustc does not warn on #[cfg(arvo_fast_math)].
    println!("cargo::rustc-check-cfg=cfg(arvo_fast_math)");
    mockspace::bootstrap::bootstrap_from_buildscript();
}
