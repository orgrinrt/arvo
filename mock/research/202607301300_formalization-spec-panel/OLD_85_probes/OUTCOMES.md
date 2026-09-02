# Outcomes, 85_probes

This file's own "compiled" claims are all recompilations of pre-existing probe source under
`79_probes/` through `84_probes/`, not new source. No new `.rs` file is added; this file records the
exact commands run this session and their raw output, so the recompilation claims in
`85_chlipala_the_closure_audit.md` are auditable the same way a new probe would be.

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved from
the repo's `rust-toolchain.toml`, confirmed by `rustc --version` inside the tree this session.

## `80_probes/probe_4b_safe_impl_refused.rs`, recompiled

```
$ cd 80_probes && rustc --edition 2021 --crate-type=lib --emit=metadata probe_4b_safe_impl_refused.rs -o /tmp/probe4b.rmeta
error[E0200]: the trait `Crosses<SomeNumeral>` requires an `unsafe impl` declaration
  --> probe_4b_safe_impl_refused.rs:22:1
   |
22 | impl Crosses<SomeNumeral> for GeneratedLowering {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: the trait `Crosses<SomeNumeral>` enforces invariants that the compiler can't check. Review
     the trait documentation and make sure this implementation upholds those invariants before adding
     the `unsafe` keyword
help: add `unsafe` to this trait implementation
   |
22 | unsafe impl Crosses<SomeNumeral> for GeneratedLowering {}
   | ++++++
error: aborting due to 1 previous error
```

Matches `80_probes/OUTCOMES.md`'s own report of this probe exactly. Closes item E of the closure
audit (section 1.1).

## `82_probes/probe_1_allones_at_real_precisions.rs`, recompiled with the binade-237 case enabled

The binary256 case in this probe is behind `#[cfg(feature = "p237")]`; a bare compile does not reach
it (silent, exit 0, no diagnostic). Recompiled with the cfg explicitly enabled:

```
$ cd 82_probes && rustc --edition 2021 --crate-type=lib --emit=metadata --cfg 'feature="p237"' probe_1_allones_at_real_precisions.rs -o /tmp/p1b.rmeta
error[E0275]: overflow evaluating the requirement `O<I<I<I<O<I<H>>>>>>: AllOnes`
   --> probe_1_allones_at_real_precisions.rs:180:24
    |
180 | const _: () = assert!(<Ones237 as Nat>::VAL == 0); // value is unrepresentable in u128
    |                        ^^^^^^^
    |
    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]`
      attribute to your crate (`probe_1_allones_at_real_precisions`)
note: required for `I<I<I<I<O<I<H>>>>>>` to implement `AllOnes`
   --> probe_1_allones_at_real_precisions.rs:93:14
    |
 93 | impl<Q: Pos> AllOnes for I<Q>
    |              ^^^^^^^     ^^^^
 94 | where
 95 |     O<Q>: AllOnes,
    |           ------- unsatisfied trait bound introduced here
    = note: 126 redundant requirements hidden
    = note: required for `I<O<I<I<O<I<I<H>>>>>>>` to implement `AllOnes`
error: aborting due to 1 previous error
```

Matches `82:74-84`'s quoted diagnostic exactly. Confirms file 82's central load-bearing claim for
section 2.3 of the closure audit: the `AllOnes` construction genuinely refuses at binary256 (p = 237)
on this toolchain, under rustc's default recursion limit.

## `82_probes/probe_2_foldexact_without_allones.rs`, recompiled bare, no flags

```
$ cd 82_probes && rustc --edition 2021 --crate-type=lib --emit=metadata probe_2_foldexact_without_allones.rs -o /tmp/p2b.rmeta
(no output, exit 0)
```

Compiles clean including its own binary256 assertions (lines 564-567: `foldexact(237,3)=239`,
`(237,257)=246`, `(237,4096)=249`, `(237,256)=245`), none of which are behind a `cfg` gate; the const
assertions are part of the compile and their being silent means they held. Confirms file 82's
replacement construction is gate-free and compiles at binary256 where the original does not.

## Cheap independent measurements (not compiled, recorded here for the same auditability)

```
$ sysctl hw.perflevel0.l1dcachesize hw.perflevel0.l2cachesize hw.l1dcachesize
hw.perflevel0.l1dcachesize: 131072
hw.perflevel0.l2cachesize: 12582912
hw.l1dcachesize: 65536
```

Matches file 81's 128 KB / 12 MB claim and file 82's 64 KB un-suffixed-key claim exactly, on this
host (`hw.model: MacBookAir10,1`, `Apple M1`, matching file 75's stated host).

```
$ cargo test --offline --workspace   (from mock/)
... 666 passed, 0 failed, 9 ignored (summed per binary from every `test result:` line)
```

```
$ grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"   -> exit 1, empty
$ grep -rln "FullRange\|UTerm\|AddWidth" mock/crates/ --include="*.rs"  -> exit 1, empty
```

```
$ git log --oneline | grep 2e2b423
2e2b423 bench: register the plan-driven and mac bitpack variants
$ grep -c "bitpack-plan\|bitpack-mac" mock/Cargo.toml mock/Cargo.lock
mock/Cargo.toml:10
mock/Cargo.lock:20
```

```
$ grep -c "[Aa]rity" 64_chlipala_the_owed_second_reads.md          -> 25  (lines matching)
$ grep -o "[Aa]rity" 64_chlipala_the_owed_second_reads.md | wc -l  -> 27  (occurrences)
$ grep -c "[Aa]rity" 55_mcsherry_typing_the_algorithm_crates.md          -> 14 (lines matching)
$ grep -o "[Aa]rity" 55_mcsherry_typing_the_algorithm_crates.md | wc -l  -> 15 (occurrences)
$ grep -c "\bcontainer\b" 78_consolidation_eight.md -> 8
```

All above run fresh this session, from the repo root or the panel directory as shown, on the pinned
toolchain where a toolchain is relevant.
