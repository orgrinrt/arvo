# 94_probes outcomes

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, resolved from `rust-toolchain.toml` inside
the tree; confirmed fresh (`rustc --version` inside the repo). Host `aarch64-apple-darwin`.
`x86_64-apple-darwin` was added with `rustup target add` for probe 1 and probe 4; it was not
installed before this file, and the install is recorded here because it changes what a later member
can reproduce without network access.

Every probe below is compiled. Probes 2 and 3 are also run. Probes 1, 4 and 5 are inspected as
emitted assembly, which is the only honest form for an instruction-count claim; no probe here makes a
timing claim, so the bench harness is not involved.

All probes build with `--edition 2021`. Under the `rustc` default (2015), `array.into_iter()` yields
references and two probes failed to compile; recorded because it cost a build cycle and will cost the
next member one too.

---

## Probe 1: the x86 form of the receipt assertion

**Owed at `91:1011-1012`**: "a compiled probe parallel to `90_probes/probe_1`, before the annotate
shape's verifier claim is called portable."

Built for both targets, `--emit=asm`, `-O`, `--crate-type=lib`. Body instruction counts, counting
everything between the label and `ret` and excluding directives and the frame pointer push/pop:

| form | target | body instructions | memory round-trips |
|---|---|---|---|
| `receipt_aarch64` (file 90's shape) | aarch64 | 3 (`mrs`, `tst`, `cset`) | 0 |
| `receipt_x86_transliterated` (RC + FTZ) | x86_64 | 5 (`movl`, `leaq`, `stmxcsr`, `testl`, `sete`) | 1 |
| `receipt_x86_mxcsr` (RC + FTZ + DAZ) | x86_64 | 5, same shape, wider mask | 1 |
| `receipt_x86_full` (+ x87 FCW) | x86_64 | 12 | 2 |

**WORKS, and the portability claim FAILS.** File 90's "three instructions" (`90:203`) is an aarch64
fact, not a price. `stmxcsr` has no register-destination form, so the x86 receipt is a store to
stack plus a reload, and the honest MXCSR field set is three fields in two disjoint regions
(RC [14:13], FTZ [15], DAZ [6]) against aarch64's two. The x87 control word is a second register with
a precision-control field (PC [9:8]) that has no aarch64 counterpart at all, and whose value decides
whether a binary64 result on an x87 path can double-round.

The transliterated form is the finding, not the count. It compiles, it looks like the aarch64
receipt, and it passes with DAZ set, which is a deployment where denormal inputs are silently zeroed
and gradual underflow does not hold. Emitted masks confirm the difference is real and not a comment:
`testl $57344` (0xE000, RC+FTZ) against `testl $57408` (0xE040, RC+FTZ+DAZ).

## Probe 2: file 90's own aarch64 mask is incomplete, on file 90's own target

Run natively. Writes each candidate FPCR bit, reads back, restores, and reports whether file 90's
mask would have caught a divergence in that bit.

```
fpcr on entry: 0x0000000000000000
file 90 receipt on entry: PASS
RMode[23:22]         latched_by_host=true  in_file_90_mask=true  receipt_would_catch=true
FZ[24]               latched_by_host=true  in_file_90_mask=true  receipt_would_catch=true
FZ16[19]             latched_by_host=true  in_file_90_mask=false receipt_would_catch=false
FIZ[0] (FEAT_AFP)    latched_by_host=false in_file_90_mask=false receipt_would_catch=false
AH[1] (FEAT_AFP)     latched_by_host=false in_file_90_mask=false receipt_would_catch=false

fpcr with outside-mask bits set: 0x0000000000080000
file 90 receipt says: PASS  (should be FAIL)

fpcr restored: 0x0000000000000000
```

**WORKS; the receipt has a hole.** FZ16 latches on this host, sits outside the mask, and the receipt
reports the IEEE default environment while half-precision underflow is flushed to zero. The design
commits to the IEEE interchange rows, binary16 among them (`91:519-520`), so this is not a
hypothetical field. FIZ and AH do not latch here (no FEAT_AFP on this silicon), so that half of the
gap is architectural rather than observable on this host, and is reported as such rather than
asserted.

## Probe 3: a preset name denotes two bundles, not one

Both ratified preset tables (`78:409-421` fixed point, `78:433-441` float, both ratified at `70b`)
transcribed cell for cell, then compared per name across the two kinds. Run.

```
Hot      TWO bundles; cells that differ by number kind: ["in-range direction", "out-of-range", "Door"]
Cold     TWO bundles; cells that differ by number kind: ["out-of-range", "Door"]
Warm     TWO bundles; cells that differ by number kind: ["out-of-range", "StoredWidth", "Door"]
Precise  TWO bundles; cells that differ by number kind: ["Door"]

  A. `Hot` rounds TowardNegative on fixed point and ToEven on float: HOLDS
  B. `Warm` stores Doubled on fixed point and Minimum on float: HOLDS
  C. `Cold` and `Precise` agree on the cells a spot-check reads: HOLDS
```

**WORKS.** All four names are kind-indexed; not one of them denotes a single row. Claim C is the
separation-requirement shape arising inside the naming question: the two presets whose divergence is
confined to `Door` are exactly the ones a reader spot-checking "does the name mean one thing" would
find reassuring.

## Probe 4: the receipt derived from the denotation, priced

The environment type carries its per-target field set as data; the receipt is a fold over it. Emitted
assembly, aarch64, `-O`:

| form | body instructions | fields checked |
|---|---|---|
| `receipt_hand_written_file_90` | 3 (`mrs`, `tst #0x1c00000`, `cset`) | RMode, FZ |
| `receipt_ieee_default` (derived) | 4 (`mrs`, `mov #29884416`, `tst`, `cset`) | RMode, FZ, FZ16 |
| `receipt_fast_math` (derived, non-zero expected) | 5 | RMode, FZ, FZ16 |

**WORKS.** Closing probe 2's hole costs **one instruction**, and the cost is entirely the wider mask
(0x1C80000, bits 19/22/23/24) not fitting an AArch64 logical-immediate encoding. The fold itself
costs nothing: no loop survives into the emitted code at either target, and the x86 instance emits
the same 5-instruction body as the hand-written honest form. Two compile-time assertions in the probe
pin the two properties that matter: the derived IEEE mask is strictly wider than file 90's and
contains it, and two environments with the same field set but different expected values do not fold
to the same check.

## Probe 5: the fold belongs on the type, not in a `const fn` called from value position

The pricing pillar's standing test (`91:117-121`) says a quantity that is a function of the type's
parameters alone belongs on the type as an associated const, because a `const fn` in value position
folds at the optimiser's discretion. Probe 4 used the `const fn` form; probe 5 checks the correct
one.

A defaulted associated const on the trait, computing the fold with a `while` loop over
`Self::FIELDS`, **compiles and const-evaluates with no feature gates** on the pinned toolchain. The
compile-time assertion `assert!(IeeeDefault::MASK == 0x1C80000)` in a `const _: () =` position holds,
which is the proof that the evaluation happened at compile time rather than at the optimiser's
discretion. Emitted body is identical to probe 4's derived form, 4 instructions.

**WORKS.** The pricing-pillar-correct shape is available on the permitted feature set today.

---

## Factual checks performed on the consolidation, before reasoning from it

- `cargo test --offline --workspace` from a clean tree at HEAD (`6519a4e`): **666 passed, 0 failed,
  9 ignored**, matching `91:43-44` exactly. The 9 ignored are 1 + 6 + 2 across three targets; the one
  `#[ignore]` attribute in the tree is `arvo/tests/fixed_point_div.rs:111`, a catalogue-red naming
  its tracked task, which is the correct form.
- The tower-absence gate reproduces: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
  --include="*.rs"` exits 1, empty (`91:41-43`).
- Toolchain reproduces: `rustc 1.98.0-nightly (57d06900f 2026-05-27)` inside the tree.
- **`91:49-53` and `91:1101-1108` are stale.** The bench-harness overwrite defect they name as a
  standing outage was fixed at commit `5dae109`, authored by op two minutes after consolidation nine
  landed. The commit adds 25 lines to `mock/benches/src/main.rs` and nothing else, so it closes the
  section-filter half of the closing artifact at `91:1025-1027` and **not** the by-reference input
  path. The footprint bench `Cold`'s intent needs is still unbuildable.
- `91:957-958` (the tautological test) reproduces and is worse than "tautological" undersells:
  `arvo-tensor/src/capacity.rs:48` defines `const CAP: Cap = cap(N);` and
  `arvo-tensor/tests/capacity.rs:14-18` asserts `<Dim<3> as Capacity>::CAP == cap(3)`. The assertion
  is `cap(3) == cap(3)` after one substitution.
