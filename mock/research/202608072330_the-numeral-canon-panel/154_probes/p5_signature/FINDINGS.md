# P5 findings: my own P4 withdrawn, and what survives it

`python3 sig.py`, output in `sig.out`. Written in phase two, after reading `110`, whose section 3
draws the distinction that shows P4 wrong.

## What is withdrawn

**F11 and F12 as stated in `p4_injectivity/FINDINGS.md` are withdrawn.** F13's criterion is withdrawn
as stated and replaced by `110`'s, which is better.

P4 reported that the index-to-primitive map is not injective, collapsing at `W = 64` (128 indices,
127 primitives), and P4b reported that the degenerate set moves with the container, `[64]` under a
u64 container and `[8, 16, 32, 64]` under a minimal one.

**Both measurements are correct and both are about my signature rather than about the primitives.**
P4's signature was one arity-1 operation: mask or clamp applied to a value **already in the
container's range**. On such an input the realisation map is never asked what to do with an
out-of-range exact result, so the overflow policy is unreachable rather than absent.

`110` names this precisely, and its own P4 was falsified by the same confusion:

- **Definitional degeneracy.** The definition of the value set and of the realisation map stops
  reading the axis. Safe to canonicalise away.
- **Reachability degeneracy.** The map still reads the axis, but no term in the *current* signature
  produces an argument on which it matters. A fact about the operation set, and it evaporates when
  the operation set grows.

## The test, and the result

The controls, stated before the run: `(64, wrap)` and `(64, clamp)` must separate under an arity-2
`add`, witnessed by concrete operands, and `(13, wrap)` and `(13, clamp)` must too. Both passed
(`sig.out:1-2`), with witnesses: `add(2^64-1, 1)` gives `0` under wrap and `2^64-1` under clamp.

```
widths where wrap == clamp, signature = arity-1 mask (what P4 used) : [64]
widths where wrap == clamp, signature = arity-2 add                 : []
widths where R itself does not read the policy (whole-line test)    : []

index points collapsing, arity-1 signature : 1 of 128
index points collapsing, arity-2 signature : 0 of 128
index points collapsing DEFINITIONALLY     : 0 of 128
```

Adding `add`, an operation any numeral must interpret, destroys the collapse entirely. And `110`'s
direct test, probing the realisation map on arguments no term need ever produce, finds the policy read
at **every** width. So the collapse was reachability, at the weakest possible signature, and there is
no definitional degeneracy anywhere in the grid I swept.

## F17. What survives, and it is not nothing

**The instrument survives as a third independent instance of the class `110` named.** `110`'s P4 was
falsified by assuming a structural degeneracy at `F = 0`; `110`'s P8 first run swept no rounding modes
and made a weak criterion look sound; and P4 here collapsed an axis that a two-operand signature
separates everywhere. Three instances, three authors' worth of care, one failure mode: **a criterion
tested against a signature too thin to reach the case it fails on.**

That is "setup that helps" (`the-test-gate.md`) occurring in the instrument rather than in a suite,
and the count matters because `110` reported two and could reasonably have been read as unlucky. It
is not unlucky. It is what this question does to anyone who asks it with one operation in hand.

**The practical form, which is what I would carry into a canon discussion:** a claim that two
primitives are the same is meaningless without the signature it is relative to, and a claim that an
axis is degenerate is unsafe unless it is checked against the realisation map on the whole line rather
than against the terms the signature happens to produce. Both halves are `110`'s; I am the second
instance of the first and an independent victim of the second.

`holds for:` W in 1..=64, policies in {wrap, clamp}, unsigned, containers in {u64, minimal
u8/u16/u32/u64}, signatures in {arity-1 mask, arity-2 add, whole-line probe of R}, F = 0, threads = 1.

## What would refute this

A definitional degeneracy in this grid: an axis the realisation map stops reading at some width. The
whole-line test found none, so refuting it means finding an argument region the test did not sample.
It samples negatives, in-range, just-over, three-times-over and `2^70`, which covers both sides of
both boundaries; a refutation would need a policy that differs only somewhere else.
