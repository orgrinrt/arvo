# Probe outcomes, SEED_THEORY_46_90

One check, run fresh this session, independent of and prior to discovering that archive files 62 and
80 had already performed the identical check: does `08_probes/README.md`'s own reproduction recipe
for the archived panel's union crate actually work.

**Question.** Archive file 57 claimed (`57:198-215`) that `08_probes/e_codegen.rs`'s dependency on
`a_union.rs`'s `pub mod spare;` and `pub mod fusion;` declarations could not be satisfied from what
is committed, because no file named `spare.rs` or `fusion.rs` exists in the archive's probe
directory, and called the resulting five-shape instruction table "not reproducible from the
committed audit trail." Archive consolidation 58 (`58:709-716`) adopted a new provenance-ground state,
`unreproducible`, on this exhibit.

**Setup.** Following `08_probes/README.md:8-11` exactly: `a_union.rs` as `src/lib.rs`,
`b_spare_pattern_decides_delivery.rs` as `src/spare.rs`, `c_split_does_not_bind.rs` as `src/fusion.rs`,
`e_codegen.rs` under `src/bin/`. All four files are byte-identical copies of the archive's own
committed probe files, renamed per the archive's own reproduction instructions (which file 57 did not
read before concluding the material was missing).

**Build**, on the pinned toolchain (`nightly-2026-05-28`, confirmed present as an installed rustup
toolchain):

```
cargo +nightly-2026-05-28 build
cargo +nightly-2026-05-28 build --bin e_codegen --release
```

**Outcome.** Both commands succeed, clean, no errors, no warnings beyond ordinary lints. The crate
compiles from exactly the material committed in `08_probes/`, laid out exactly as `08_probes/
README.md` already instructs.

**Verdict.** File 57's "cannot be reproduced by anyone from what is committed" is false. The
reproduction recipe was in the same directory the whole time. This is the third independent instance
of this correction (mine, archive file 62's `62_probes/rebuild_union.sh`, archive file 80's
independent second primary-source-style re-derivation), satisfying this workspace's preference for
three or more independent instances of evidence.

**What this does not establish.** The specific "28.45 seconds at eight bits, refusal at nine"
wall-clock figure `unstable-features.md` cites traces to a different probe (a width-ceiling sweep,
not this union crate), sits in archive file 8, outside this dispatch's assigned slice (46-90), and
was not re-derived here. Archive file 62 (`62:44-89`) did independently rebuild that sweep and
reproduced its qualitative structure (cost quadruples per bit, refused at nine bits with the
identical diagnostic) with different absolute wall-clock numbers on different hardware; that
narrower claim is not re-verified by this probe and remains a question for whoever holds file 8's
slice.

Source files here are byte-identical renames of committed archive material
(`202607301300_formalization-spec-panel/08_probes/a_union.rs`,
`b_spare_pattern_decides_delivery.rs`, `c_split_does_not_bind.rs`, `e_codegen.rs`), reproduced here
under their reproduction-recipe names so the build in this directory is self-contained and does not
require cross-referencing the archive's probe directory to run.
