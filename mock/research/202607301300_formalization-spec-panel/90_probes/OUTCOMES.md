# 90_probes outcomes

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`, every command run from the repo root inside the tree. HEAD at `9ce1fd8`.

## Gates

Canon gate, run fresh:

```
grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"   -> exit 1, empty
grep -rln "FullRange\|UTerm\|AddWidth" mock/crates/ --include="*.rs"  -> exit 1, empty
```

Test gate, summed per binary from every `test result:` line:

```
cd mock && cargo test --offline --workspace
passed: 666  failed: 0  ignored: 9
```

Matches files 81 through 89 exactly. The one disqualifying test on record,
`arvo-tensor/tests/capacity.rs:14-18`, confirmed by reading the body and the impl this session:
the impl at `arvo-tensor/src/capacity.rs:49` is `const CAP: Cap = cap(N)`, so all three assertion
lines reduce to `cap(k) == cap(k)` after monomorphisation. The rest of that test file is real
(`filled_populates_every_slot_at_exact_length` asserts length and content against independent
values). Deletion of the one test, not improvement, exactly as `78:874-876` carries it.

## Probe 1: the receipt assertion is buildable, gate-free, and detects divergence

`probe_1_the_receipt_assertion_is_three_instructions.rs`. Compiled with
`rustc --edition 2021 -O`, zero feature gates, run on this host:

```
fpcr initial: 0x00000000
receipt assertion PASSES on fresh process
fpcr after FTZ set: 0x01000000
receipt assertion DETECTS the divergence
restored; receipt assertion passes again
```

Separation statement per `86b`: the probe separates the declared control state from the live one
by deliberately diverging them (FTZ set, re-checked, restored), so the assertion is shown to fail
where the name's claim is false, not only to pass where it is true. `objdump -d` counts 7
`mrs`/`msr` instructions across the whole binary (four reads, two writes, panic paths); a single
check is one `mrs`, one masked compare, one branch, consistent with `63:604-605`'s "three
instructions of cfg-gated inline assembly". FPCR fields (RMode bits 23:22, FZ bit 24) are a
secondary read of the Arm ARM; the fresh-process value 0x0 (RNE, no FTZ) is the measured fact.

## Record checks (reads and greps, not compiles; commands as run)

**File 37's probe computes the per-value-moved reading.** Read at source:
`37_probes/probe_1_the_ladder_is_a_view_lattice.rs`, `add` (lines 169-210): `resolve` (the only
place `e: 1` is set) is called only under `s > p.ihi` or `s < p.ilo`; the in-range branch carries
`e: be` unchanged. So the ratified finest-view table at `37:171-179` is a reading-B measurement,
confirming file 89's characterisation from the source rather than from its prose.

**File 43's probe states the per-application reading, names the fork, and mis-grounds it.** Read
at source: `43_probes/probe_5_the_roundtrip_law_and_its_view.rs:15-24` states "an event is counted
per quantiser APPLICATION, not per value actually moved", grounds it on `40:279-287`, and itself
names the consequence of the other reading ("the law would sit one lattice point higher").
`40:281-283`, read fresh: "**`IS_EXACT` alone does not trivialise an operation's grade monoid;
`IS_EXACT` and `Total<Op>` together do.**" A statement about when the monoid is trivial, nothing
about whether a nontrivial monoid's content is value-dependent. File 89's citation-gap finding
confirmed against both sources.

**The `IeeeDefault` trail.** `grep -rn "IeeeDefault" *.md`: hits in files 59, 62b, 63, 64, 67b,
68, 78 and nowhere later; the last is `78:855` (op item 5). Zero occurrences in files 79 through
89, confirming the consolidation's "untouched this stretch" for the bundle. `67b:180-182` is the
persona-checkpoint adoption of the naming principle; `78:829-838`'s walkthrough of op's
confirmations does not list it, so the principle's provenance is persona-tier, unconfirmed.

**The blast-radius change.** `78:441` (ratified float table): `Warm`'s door is `HostFloat<E>`;
`78:470-472`: four cells of sixteen reach hardware. File 64's name analysis (`64:406-480`)
predates `70b` and was written when one cell did.
