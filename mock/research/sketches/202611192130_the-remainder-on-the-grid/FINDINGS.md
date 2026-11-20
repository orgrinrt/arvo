# The remainder on the grid

**Outcome:** WORKS

Copied by the dispatcher from the section "The sketch, as its findings record" of
`mock/research/202611192115_the-remainder-panel/donald-knuth.md`, which the seat wrote when the harness
refused it this file. The seat's words follow unchanged.

A standalone crate with its own `[workspace]`, depending on nothing, every value an exact rational formed
in it. It unblocks the definition and region of that file, the refutation of the exponent as the dividing
line, and the refutation of the composition over the operands' signature. The directory holds
`src/main.rs`, `Cargo.toml`, `Cargo.lock`, the passing run `out.txt` and one output per plant.

Hypothesis: the seven laws and the region table of that file, at the instances swept.

Attempt 1 stated the double-rounding claim as "the nearest modes miss at every negative exponent" and
stopped:

```
  radix 10 e -2: [(TowardZero, 0), (Floor, 0), (Ceil, 0), (HalfUp, 0), (HalfEven, 0), (StochasticDown, 0), (StochasticUp, 0)]

thread 'main' panicked at src/main.rs:446:17:
HalfUp at radix 10 e -2: expected the composition to miss
```

A miss needs `a / b` within `Q / 2` of a half without being one; the closest a non-tie gets is
`1 / (2 |s_b|)` at an odd divisor slot, so it needs an odd `|s_b| >= r^-e`, which widths up to six do not
reach at `10^-2`. The claim was restated and the widths taken to eight.

Attempt 2, with that condition, stopped at radix three:

```
thread 'main' panicked at src/main.rs:455:25:
assertion `left == right` failed: HalfUp at radix 3 e -2 over (0, 15)
  left: false
 right: true
```

At an odd radix the half is not a grid point but the midpoint of two, so the first rounding keeps the side
of the half and nothing misses. The condition became: an even radix, and an odd divisor slot with
`|s| >= r^-e`.

Attempt 3 is the committed source. Every claim held and every control fired. The three plants each fail
on the check planted against: `plant-quotient-on-the-grid` (a quotient in eighths) at "TowardZero: a
remainder off the grid at a whole phase"; `plant-sign-of-the-dividend` (floor's quotient rounded toward
zero) at "Floor: a remainder of the wrong sign"; `plant-float-checker-accepts-all` at "the membership
check accepts every sum, so it is blind". The numbers, from `out.txt`:

1. Whole phase (`whole_phase`, `main.rs:230`), radixes 2, 3 and 10, exponents -3 to 2, phases 0, 1 and
   -2, signed and unsigned widths one to five, 141876 pairs per mode: zero off the grid, zero
   disagreements with the value-index form, zero past the bound, zero of the wrong sign, zero leaving a
   range that holds the zero under `toward_zero` or `floor`, zero leaving a phase-zero signed range under
   any mode. The integer quotient is not a member of the operands' format at 53162 to 65562 pairs per mode.
   Option b's remainder agrees at every pair at exponent zero and phase zero and departs at 75200 to 85366
   pairs per mode overall. Unsigned ranges are left under `ceil` at 56070 pairs, `half_up` 27252,
   `half_even` 25092.
2. The quotient's span (`quotient_span`, `main.rs:349`): `max |q|` is `2^(w-1)` signed and `2^w - 1`
   unsigned at widths one to six, one value across every radix and exponent; the `Fix<8, -4>` example as in
   that file.
3. Fractional phase (`fractional_phase`, `main.rs:390`), phases 1/2, -1/2, 1/3, -2/3, 3/4, 1/5, radixes
   2 and 10, exponents -2, 0, 1: on the grid at 254208 mode-pairs, off at 433248, on exactly where `d | q`,
   every format with a pair off, the exact remainder always on the lattice of quantum `Q / d`.
4. Double rounding (`double_rounding`, `main.rs:434`), widths one to eight: zero misses for the directed
   modes at every negative exponent; for `half_up` and `half_even`, 15470 and 15476 over a signed byte at
   `2^-1`, 56 and 56 at `10^-2`, none at `3^-2` at any width, and zero exactly where the condition says;
   at exponent one every mode misses at tens of thousands of pairs.
5. Composition on the operands' signature (`composed_on_the_signature`, `main.rs:496`): under saturation
   `toward_zero` 0 wrong, `floor` 464, `ceil` 360, `half_up` 196, `half_even` 220, counted only at pairs
   whose true remainder is a member; under wrapping 0 wrong for every mode.
6. Euclid (`euclid`, `main.rs:543`): 3906 pairs, the floor's remainder by a positive divisor and the
   ceiling's by a negative one, never negative.
7. Toy floats (`magnitude_indexed`, `main.rs:587`), radix 2 at precisions 2 to 4, radix 10 at 1 and 2,
   radix 3 at 2, subnormals included: `toward_zero`, `half_up` and `half_even` give a member at every pair;
   `floor` and `ceil` miss (992 pairs each at radix 2, precision 4; 46620 each at radix 10, precision 2).
   The control finds non-member sums in every format.

What it does not establish: the sweeps are instances, and the general statements rest on the derivations in
that file. It runs no arvo step, so the shipped machinery's four-limb bounds, its dither rule for
`stochastic` (swept here only as its two outcomes) and width 64 are untested. The toy floats carry no
infinity, no signed zero and no overflow.

