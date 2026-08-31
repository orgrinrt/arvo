# Sketch: can an open trait check its own implementors at compile time

**Hypothesis.** `Slots` is open, so an outside crate can implement it incoherently and nothing in the
design notices. If that is true, three sentences claiming totality over it are false. And if a
defaulted associated const can refuse such an implementor at compile time, the trait can stay open
while admission becomes a check, which is what the canon says admission is.

Cited by `mock/design_rounds/202609010001_topic.a-totality-claim-over-an-open-trait.md`.

## Outcome, both arms

**The finding is CONFIRMED.** `without_the_check.rs` is the shipped shape: the rogue implementor
supplies `MIN = 4611686018427387904`, `MAX = -4611686018427387905`, `WIDTH = 63`, compiles with no
features and no patches, and `slot_count` returns `-9223372036854775808`. Those values are the
reviewer's, used verbatim so this is a reproduction rather than a similar construction.

**The fix WORKS.** `with_the_check.rs` is the same file plus a defaulted `ADMITTED` const forced by
`slot_count`. It is refused:

```
error[E0080]: evaluation panicked: slot range is inverted: MIN exceeds MAX
  evaluation of `<Rogue as Slots>::ADMITTED` failed here
```

At compile time, on a trait anyone may implement, for an implementor the crate cannot see.

## What must fail

The two files differ by one line, the `let () = S::ADMITTED;` in `slot_count`. If the checked arm ever
compiles, the mechanism does not work and the design cannot claim admission is checked. If the
unchecked arm ever refuses, the finding was not reproduced and the diagnosis is wrong.

Both are in `output.txt` from one run, so the pair is the control for each other.

## What this does not establish

That the mechanism catches an implementor whose incoherence no assertion names. It catches what the
assertions test for, and nothing more, which is why the assertions are the design decision rather
than the mechanism.

It also does not fire for a consumer that reads `MIN` and `MAX` directly without going through a
function that forces the const. The design's own functions all force it; a consumer reaching around
them is reaching around them.

## Toolchain

`rustc --edition 2021 -O`, the pinned nightly, no cargo and no dependencies.

## The limitation, found while trying to pin this and sharper than the fix

**The refusal fires at codegen and not at `cargo check`.** Measured on a real consumer crate:

```
cargo check   ->  Finished, no error
cargo build   ->  error[E0080]: evaluation panicked: slot range is inverted ...
```

A post-monomorphisation const assertion is evaluated when the instantiation is codegened, and
`cargo check` skips codegen. So a consumer whose editor and CI run `check` sees nothing, and only a
real build refuses.

**Two consequences, both recorded rather than worked around.**

`trybuild` cannot pin this case. It checks rather than builds, so it reports the case as compiling and
its own diagnostic is that the test succeeded when it should have failed. The compile-fail case for
this was written, found not to work, and removed rather than left as a green test asserting nothing.
The width-above-the-bound case stays, because a missing impl is a type error and does fire at check.

**And the claim has to be qualified.** "Refused at compile time" is true of a build and false of a
check, and writing it unqualified would be the same shape as the three totality sentences this round
is removing. What is pinned instead is the law: `is_admissible` reports on a construction without
forcing the const, so a permanent unit test asserts the design can tell the two apart, at check time
and at build time both.

**What the guarantee actually is**, stated exactly: an inadmissible range cannot reach a produced
binary, and it can reach a passing `cargo check`.
