# Outcome: WORKS, with a corrected vehicle

Hypothesis under test: a declarative `macro_rules!` muncher can read a
decimal literal and emit the numeral tower's own value-unique encoding.

**FAILS, and not merely "hairy."** `61_probes/probe_1` compiles the
attempt: an integer or float literal is one atomic lexer token, and no
`macro_rules!` fragment specifier or matching technique can decompose it
into digit characters. `61_probes/probe_2` compiles the fallback (drive
the peel via a `u64` const-generic parameter instead of a token match) and
finds it walled identically to the design's own exponent case (58:143-148):
neither the bare language nor `min_generic_const_args` admits a generic
const in a recursive const-position. Both routes are dead, not merely slow.

**Fallback taken, per the brief's own stated order: a compile-time-only
proc-macro crate**, `crates/numeral_pm.rs`, no external dependencies
(`syn`/`quote`/`proc-macro2` are not needed; the sysroot's own
`proc_macro` crate is enough), matching the workspace's own precedent
(`notko-macros-core`, a proc-macro crate declaring its std dependency
honestly) while being lighter than it.

The vehicle WORKS: `crates/consumer_matrix.rs` compiles and passes 923
assertions (hand-picked boundary/near-miss cases, 900 exhaustive
three-digit magnitudes), each checked against `Bias::NUM`/`Bias::DEN`
read back through the type-level `Pos::VAL`, not merely "it compiled."
One real bug (a numerator-inflation error in the rational-denominator
combining step) was caught by this test before this file was written, per
`a-test-that-cannot-compile-is-the-finding.md`'s sibling discipline for
runtime assertions: the strict test found the defect, not review.

Two ceilings, not one, both compiled precisely (`61_probes/probe_4`,
`probe_5`), and they are not the same wall the consolidation's "roughly
`2^127`" (58:618) compresses them into: a `Pos` is nameable up to nesting
depth 128 (structural, the trait-solver's recursion limit) but its `VAL:
u64` cannot be read past 64 bits (const-eval overflow), a much tighter and
independent wall. The vehicle reports the correct one, using the decimal
number it already has host-side, never the encoding
(`crates/consumer_ceiling_readout.rs`).

The face survives its own declaration and decays at the first operation
generic over the raw encoding, reproducing file 56's finding at this
vehicle's own emitted types rather than at a hand-written stand-in
(`crates/consumer_diagnostic.rs`, `consumer_diagnostic_decay.rs`).

One structural finding beyond what was asked: a macro living in its own
crate cannot ever satisfy a trait sealed via a private supertrait it did
not originate inside (`61_probes/probe_3`), which sharpens file 56 section
4.3's "bridge trait" language from a stylistic preference into the only
structurally available shape once any carrier the face touches is sealed.

Full writeup: `../../202607301300_formalization-spec-panel/
61_amin_the_notation_vehicle.md`.
