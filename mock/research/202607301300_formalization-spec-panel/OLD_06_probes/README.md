# Probes for panel file 06, the consumer surface

Five probes, all compiled under `nightly-2026-05-28`, the pinned toolchain. Each is a standalone
crate body; `a_alias_render.rs` additionally needs a path dependency on `arvo`, `arvo-storage`,
`arvo-strategy` and `notko`, and the rest need nothing at all.

To run one of the self-contained ones: `cargo new --lib pN`, drop the file in as `src/lib.rs`, pin
the toolchain, `cargo build`. The interesting output is the compiler's, not the program's.

The recorded output of each is quoted in `06_muratori_the_consumer_surface.md` at the section named
in the file's own header comment. Where a probe deliberately fails to compile, that failure is the
result and the file says so.
