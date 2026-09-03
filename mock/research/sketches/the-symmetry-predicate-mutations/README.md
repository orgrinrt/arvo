# The symmetry predicates, mutated on purpose

Two mutations of `arvo-format/src/symmetry.rs`, each with the test output it
produced. Neither is reproducible from the tree, which is exactly why both are
here: a mutation run cited in prose and not committed is a claim resting on
nothing, and the previous round cited one that way.

The hypothesis both answer is the same one, and it is the only interesting
question about a predicate nothing in the crate reads. Do the arms actually run
the machinery, or do they read the declaration back and agree with it? A
declaration read back agrees with every mutation of itself.

## Mutation one: half-even reads the sign

`WORKS`, in the sense that the mutation is caught.

```rust
        Mode::HalfEven => {
            Behaviour {
-               reads:    Reads::Parity,
+               reads:    Reads::Sign,
                when:     When::AtATie,
                reflects: Bool::of(true),
            }
        },
```

Two arms fail, `out/parity_reads_the_sign.txt`:

```
test symmetry::tests::the_classification::the_classification_of_every_mode_agrees_with_what_the_map_reads ... FAILED
test symmetry::tests::the_cross::the_map_relocates_exactly_where_the_predicate_says ... FAILED
```

The cross fails with `Signed HalfEven Wrap Restriction { negative_positions:
false, negative_translations: false, ties: true }: the predicate says true and
the map says false`, which is the unsound direction. A cell where the predicate
licenses a law the map does not honour is what an arm gated on it would compute
a wrong answer from, so it is the direction the cross exists to catch and the
one worth seeing fail.

This reproduces what the previous round's source changelist reported in prose,
in the same words, on the same restriction.

## Mutation two: the completion region's low side loses its disjunct

`WORKS`, and it is the one that says why the arm it fails was written.

```rust
-            let low_side = reach
-                .reaches_below(lowest)
-                .not()
-                .or(reach.reaches_a_positive_translation().not());
+            let low_side = reach.reaches_below(lowest).not();
```

One arm fails, `out/low_side_disjunct_removed.txt`:

```
test symmetry::tests::the_reach::a_translation_band_with_no_positive_translation_licenses_a_low_excursion ... FAILED
TowardZero refused a low excursion that no translation in the band can undo
```

One, and it is the arm this round added. The cross's 432 cells all pass under
this mutation, because `cell_reach` hands every cell the range's highest slot as
its upper translation bound and that is positive on all three ranges, so
`reaches_a_positive_translation()` is true everywhere and the disjunct never
carries a verdict. The high side's mirror is exercised both ways through the
`negative_translations` axis, which is what made the gap easy to miss: half the
symmetric pair was covered.

## Reproducing either

Apply the diff, run `cargo test -p arvo-format --lib` from `mock/`, restore the
file. Restore by writing rather than by moving a backup over it: a backup's mtime
is older than the object built from the mutant, so cargo keeps the stale binary
and the run afterwards reports the mutation still failing. Both files here were
restored that way and the module's checksum was compared against the value taken
before the first mutation.
