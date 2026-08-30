# RUN.md, probes for file 43

Every command below was run from the panel directory unless noted.
Toolchain, stated rather than assumed:
```
rustc 1.98.0-nightly (57d06900f 2026-05-27)
```

Feature-gate census across every Rust probe here (must be zero on each):
```
$ grep -c '#!\[feature' *.rs
p2_which_coordinates_each_derivation_reads.rs:0
p4_nesting_and_the_flattened_capacity.rs:0
```

## s1: the composition-word census
```
$ python3 43_probes/s1_composition_word_census.py > 43_probes/s1.out
(full output in s1.out; totals reproduced here)

==============================================================================
GRAND TOTAL
==============================================================================
  AGGREGATE  47
  BIND       37
  FUNC       50
  UNCLEAR    105

The founding sentence, from 00_brief.md:
  00_brief.md:8: Arvo has no canon. This panel writes it: the primitives become named compositions over one format

Op's sentence, from 32_op_arvo_adapts_to_the_cores_it_finds.md:
  32:73: > for things that compose to bigger units than just numerals alone. But we need this base to work, to
```

## p2: which coordinates each derivation reads
```
### MAIN ARM (must build)
exit=0

### ARM negcontrol (must NOT build: the value map does read the grid)
error[E0080]: evaluation panicked: assertion failed: value_numer::<GridA>(5) == value_numer::<GridB>(5)
   --> p2_which_coordinates_each_derivation_reads.rs:414:5
    |
414 |     assert!(value_numer::<GridA>(5) == value_numer::<GridB>(5));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `_VALUE_MAP_IGNORES_GRID_IS_FALSE` failed here


### ARM lawneg (must NOT build: a wrapping strategy has no absorbing top)
error[E0277]: the trait bound `Wrap: AbsorbingTop` is not satisfied
   --> p2_which_coordinates_each_derivation_reads.rs:335:38
    |
335 |     tropical_fold_site::<GridA, N13, Hot>();
    |                                      ^^^ unsatisfied trait bound
    |

### ARM sameneg (must NOT build: SameAs is not vacuous)
error[E0277]: the trait bound `S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>>>>: SameAs<S<S<...>>>` is not satisfied
   --> p2_which_coordinates_each_derivation_reads.rs:429:9
    |
429 |         <Num<GridA, N13, Hot> as AccReach<Cap<8>>>::R,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
```

## s3: capacity under nesting
```
$ python3 43_probes/s3_capacity_composes.py > 43_probes/s3.out
(full output in s3.out; headline reproduced here)
Q1/Q2  over m,n in [1,64]  ->  4096 pairs
   FLAT strictly wider than NESTED : 0
   equal                           : 2895
   NESTED strictly wider than FLAT : 1201  (29.3%)
   slack histogram (extra bits)    : {1: 1201}
   worst case: m=3 n=5 -> nested adds 5 bits, flat adds 4, slack 1
Q3  sufficiency of the FLAT accumulator for a nested traversal
   checked (w,m,n) triples          : 8192
   per-row intermediate overflows   : 0
   final total overflows            : 0
   NEGATIVE CONTROL, one bit narrower, overflows: 6502   (must be large, else the check above is vacuous)
```

## p4: nesting and the flattened capacity
```
### MAIN ARM (must build)
exit=0

### ARM tightneg (must NOT build: flat and nested disagree at 3x5)
error[E0080]: evaluation panicked: assertion failed: <<Flat3x5 as NestedAccReach>::R as Nat>::V ==
                  <<Flat3x5 as FlatAccReach>::R as Nat>::V
   --> p4_nesting_and_the_flattened_capacity.rs:314:5
    |
314 | /     assert!(
315 | |         <<Flat3x5 as NestedAccReach>::R as Nat>::V
316 | |             == <<Flat3x5 as FlatAccReach>::R as Nat>::V
317 | |     );

### ARM missingcap (must NOT build: the flattening is load-bearing)
error[E0277]: the trait bound `Run<Run<Num<S<S<S<S<S<S<S<S<S<...>>>>>>>>>, ...>, ...>, ...>: TotalCap` is not satisfied
   --> p4_nesting_and_the_flattened_capacity.rs:327:14
    |
327 |     assert!(<<Unknown as FlatAccReach>::R as Nat>::V > 0);
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
```

## s5: which coordinates survive which operation
```
$ python3 43_probes/s5_which_coordinates_survive_which_operation.py > 43_probes/s5.out
(full output in s5.out; the two load-bearing blocks reproduced here)
Q2  which coordinates the raw (stored-integer) implementation reads
    on a SAME-GRID operation.  A coordinate is 'read' when the raw
    result depends on it.

    add: raw result independent of adjustment      : True
    add: raw result independent of bias            : False
    add: raw result independent of canonical expt  : True
    mul: raw result independent of adjustment      : False
    mul: raw result independent of bias            : False
    mul: raw result independent of canonical expt  : False
    add: raw result is an integer index in 833 of 1323 (grid, pair) cases

Q3  the n-ary sum, with the trip count n dynamic
    B == 0 : predicted step correct 9, wrong 0
    B != 0 : predicted step correct 8, wrong 10

    At zero bias the product's grid step is A^2 r^2e, a function of
    the operands' coordinates alone.  At nonzero bias it is not:
    the cross term A*B*(k1+k2) puts A*gcd(A,B)-shaped quantities in
    the difference set, so the derived step depends on an arithmetic
    relation between two coordinates rather than on either.

    addition, by contrast: step is A*r^e and origin is 2B, in 27 of 27 grids
```

## p6: citation check
```
$ python3 43_probes/p6_citation_check.py
```
Result recorded in p6.out. It opens every file:line in the document and
tests its content against a word the claim requires, and cross-checks its
own table against the citations present in the document.

## p7: does the capacity bound survive lowering

A qualitative assembly read. Not a bench, no timing, and the file says so in
its own header and in section 6.4 of the document.

```
$ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -O --emit asm \
        p7_does_the_capacity_bound_survive_lowering.rs --out-dir build
$ python3 p7_asm_census.py > p7.out
```

The emitted assembly is committed as `p7.s` beside the source, so a reader can
check the census rather than rerun it. `p7_asm_census.py` carries, in its own
docstring, the instrument defect it started with: its first failure-path pattern
contained `_failed`, which does not match `slice_index_fail`, and it reported
zero for the one arm the conclusion turned on.
