# The old `half_up` tie rule, planted back into `round_slot`

One mutation of `arvo-format/src/apply.rs`, with the test output it produced.
It is not reproducible from the tree, because the tree carries the fixed rule,
which is why the output is here.

The planted tie rules in `apply/tests/the_broken_maps.rs` put the old rule into
a map the sweeps are handed, so they check that the sweeps report it. What they
cannot check is that the rest of the suite, the parts reading `adapt` and the
classification rather than a `Map`, would notice the old rule in the shipped
path. This mutation answers that.

## The mutation

`WORKS`, in the sense that the mutation is caught.

```rust
        Mode::HalfUp => {
-           if twice >= den {
-               up
-           } else {
-               down
-           }
+           if twice > den {
+               up
+           } else if twice < den {
+               down
+           } else if exact.is_negative() {
+               down
+           } else {
+               up
+           }
        },
```

That is the arm the crate carried before the ruling: a tie at a negative
position goes down, away from zero, and every other tie goes up.

## What failed

`out/old_rule_in_round_slot.txt`, run as `cargo test -p arvo-format
--no-fail-fast` from `mock/`, debug profile. Twenty-seven tests fail: 24 in the
library and 3 in `tests/matlab_fi_parity.rs`. The compile-fail suite and the
two other suites the command runs stay green, which is expected, since none of
them reads the rounding.

The failures fall in every place the ruling reaches:

- the pinned tie arm and the bottom-of-the-index hand arm;
- both oracle sweeps and the tie sweep, at every declared signature and at the
  map level;
- the reporting arms of the narrow oracle sweep and of the tie sweep, because
  every broken map is built over `round_slot` and so carries the mutated arm
  too, and the first break each sweep finds is the mutant's rather than the one
  the arm expects: `past_the_bottom_pins_high_off_the_bottom` is found inside
  the index, and `half_even_ties_to_odd` is found under `HalfUp`;
- the classification derivation, the partition and the cross, since `HalfUp`
  now reads the sign at a tie while `behaviour_of` says it reads nothing;
- the associativity verdict tables;
- the MATLAB arms, since `HalfUp` stops being MATLAB's Nearest and becomes
  MATLAB's Round.

The wide sweep's reporting arm stays green, because it asks only that each
broken map disagrees with the oracle somewhere, and all of them still do.

## What it does not show

One mutation, one instance. It says the suite notices this rule in this
function. The other wrong tie rules are covered by the planted maps through
the sweeps and not by a mutation of the shipped function; ties down, ties
toward zero and ties to odd were not planted into `round_slot`.
