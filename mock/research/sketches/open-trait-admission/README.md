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
