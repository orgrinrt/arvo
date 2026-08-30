# typenum counts: what is checkable and what is not

`11` cites typenum's generated constants as evidence that arvo's three declarations are what Rust's
position costs, reporting **1148 rows and 4758 generated lines**. The `14` checkpoint re-ran both
commands independently and confirmed both figures exact.

**The artifact itself cannot be committed here, and this file says so rather than pretending
otherwise.** typenum generates `consts.rs` at build time from `src/gen.rs` into `OUT_DIR`; it does
not exist in the published crate source. What sits in the registry is the generator, not the output.

So the claim rests on a reproduction rather than on a committed artifact. Under the rule that
evidence lives in the repo or it never happened, that is weaker than a committed probe, and the
honest classification is: **reproducible on demand, twice confirmed, not archived.**

## Reproduction

The crate in the local registry at the time of writing is `typenum-1.20.0`. Build it, then count in
the build output directory:

```
cargo build -p typenum
find target -path '*typenum*' -name consts.rs
grep -c 'pub type U[0-9]' <that path>
wc -l < <that path>
```

The generator is at `src/gen.rs` in the published source and is committable if a later reader wants
the input rather than the output.

## Why this matters beyond one citation

Two other numbers in tonight's files rest on paths outside the repository. This one was caught
because a checkpoint went looking. The general remedy is the one the rules already state: a claim
that cannot be checked from the repository is worth less than one that can, whatever its provenance,
and saying which kind it is costs one sentence.
