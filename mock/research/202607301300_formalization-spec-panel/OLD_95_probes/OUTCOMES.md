# 95_probes outcomes

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside
the tree immediately before the first compile. Tree at `6519a4e` with files 93 and 94 untracked
(`git status --short` shows exactly four untracked paths, all under the panel directory).

No bench was run. The orchestrator's artifact-destroying defect is fixed (`5dae109`, a section
filter, 25 lines to `mock/benches/src/main.rs` and nothing else), but the by-reference input path a
footprint bench needs is not in that commit, so a bench that must exceed cache is still unbuildable.
Nothing below is a timing claim. Everything below is a compile outcome or an executed program's
stdout.

## probe_1: the niche-typed door is safe and is not domain-total

```
$ rustc --edition 2021 -O probe_1_the_niche_door_is_safe_and_not_domain_total.rs -o out/probe_1
$ ./out/probe_1
A (domain fills the niche): CARD=65535 inhabitants=65535 decoded=65535 orphaned=0 door_domain_total=true
B (bounded numeral, 2^13 values): CARD=8192 inhabitants=65535 decoded=8192 orphaned=57343 door_domain_total=false
B: after one safe store through the niche-typed door, decode() = None (carrier raw = 60000, debias = 59999)
door-total const equation (CARD == 2^w - 1): A=true B=refused, see probe_1b
```

Zero `unsafe` constructs in the file. Both sweeps walk every one of the carrier's 65,535
inhabitants through the door, so neither result is a sample.

The two instantiations are one model. At A the door places the carrier only on inhabitants the
decode answers for; at B a single safe store lands on one of 57,343 inhabitants it does not. The
57,343 figure reproduces `92_probes/probe_2:40-41` exactly, which is the point: file 92 compiled
this number in its section 1.3 for a different argument and tested the door in section 2.3 at
instantiation A only.

## probe_1b: the close, and it refuses per door rather than per type

```
$ rustc --edition 2021 probe_1b_the_close_refuses_at_declaration.rs -o out/probe_1b
error[E0080]: evaluation panicked: assertion failed: CARD == NICHE_INHABITANTS
  --> probe_1b_the_close_refuses_at_declaration.rs:21:28
   |
21 |     const DOOR_TOTAL: () = assert!(CARD == NICHE_INHABITANTS);
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `Bounded::<8192>::DOOR_TOTAL` failed here
...
note: the above error was encountered while instantiating `fn Bounded::<8192>::typed_mut`
  --> probe_1b_the_close_refuses_at_declaration.rs:34:13
```

Same error class as the level-ordering refusal at `83_probes/probe_3`. The granularity is worth
recording: the refusal fires while instantiating the *door*, not while declaring the type, so
`Bounded<8192>` remains a declarable, constructible, decodable numeral and only its mutable door is
refused. That is the correct granularity for the finding, since the numeral is not the thing at
fault.

## probe_2: the column granule reopens the gap, with no `unsafe` anywhere

```
$ rustc --edition 2021 -O probe_2_the_column_granule_reopens_the_gap.rs -o out/probe_2
$ ./out/probe_2
W_S=5 N=12 used_bits=60 bytes=8 tail_padding_bits=4
unsafe blocks in this file: 0
per-element reads agree after the write: true
fresh   digest: 0xbc6daaf74a02ab99
dirtied digest: 0xbc6deaf74a031859
digests agree: false
theorem as worded (no raw accessor below the FIELDS' width): satisfied by this type
theorem quantified per byte-owner (no accessor below the COLUMN's write granule): violated by bytes_mut
```

`grep -cE "unsafe (\{|fn|impl|trait)"` returns 0; the two textual occurrences of the word are both in
prose. This compiles file 92 section 2.1's fourth bullet, which that file offered as reasoning.

## probe_3: the perimeter has no expressible form as a bound

```
$ rustc --edition 2021 -O probe_3_the_perimeter_is_not_a_bound.rs -o out/probe_3
$ ./out/probe_3
granule(Honest)    = 16
granule(Dishonest) = 16
marker satisfied by both: true true
Honest    value=7 raw=0x0007
Dishonest value=7 raw=0xe007
the marker compiled, the perimeter did not hold, and rustc said nothing
```

The compile is clean, and the clean compile is the finding. The strongest form the language permits
(an `unsafe trait` whose documented contract is the whole perimeter) is satisfied by a type with a
`pub` field, and the safe write the contract forbids compiles with no diagnostic. Nothing on the
permitted feature set sees a field's visibility: no `TypeId`, no reflection, no full
`specialization`, and `min_specialization` dispatches on types rather than on their declarations.

A derive macro could check the syntactic half (this struct has no `pub` field) because it is handed
the item. It cannot check the transitive half (no method anywhere hands out a reference below the
granule) because it is handed one item. So the perimeter is prose by necessity, not by preference.

## Corpus checks performed for the deliverable

```
$ grep -rn "safe surface" *.md          # 14 hits across 87, 88, 90b, 91, 92
$ grep -rn "safe surface is\|safe surface means\|safe surface denotes" *.md   # 0 hits
$ grep -rn "raw accessor" *.md          # 11 hits across 05, 87, 88, 90b, 91, 92
$ grep -rn "designated" 90b_*.md 91_*.md # 4 hits, no definition
```

`91:612-615` and `90b:100` are ratifying text; "safe surface" and "raw accessor" appear in them and
are defined nowhere in the corpus. `91:846-850` is ratifying text; "designated" appears in it and is
defined nowhere, which is the ambiguity file 94 section 4 found by a different route.

Ratified tables re-read at source rather than through file 94's transcription
(`78:409-421` fixed point, `78:433-441` float): `Hot` is `TowardNegative` on fixed point and `ToEven`
on float; `Warm` is `doubled` on fixed point and `minimum` on float; `Cold`'s out-of-range is `clamp`
on fixed point and `far point` on float; `Precise` is `Refuse` on both. File 94's probe 3 models
`Clamp` and `FarPoint` as distinct variants and reports `Cold` as differing on out-of-range, which
its own prose at `94:172-174` contradicts.

## Gates

`cargo test --offline --workspace` from `mock/`, summed across every `test result:` line: 149
binaries, **666 passed, 0 failed, 9 ignored**, matching `91:43-44`.

Canon gate: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty.

`mock/crates/arvo-tensor/tests/capacity.rs:14-18` re-read at source against
`mock/crates/arvo-tensor/src/capacity.rs:48` (`const CAP: Cap = cap(N);`). The assertion substitutes
to `assert_eq!(cap(3), cap(3))`. It is what the registry says it is (`91:957-958`) and what file 94
section 0 says it is. Out of this panel's scope to delete; counted in the 666.
