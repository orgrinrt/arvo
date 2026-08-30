# 33_probes outcomes

Five probes, all compiled against `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the pin
`rust-toolchain.toml` names and the pin files 31 and 32 gate against. Every assertion below is a
`const` assertion: the compiler either accepted the file or refused it, so a passing build is the
result and there is no runtime pass to misread. Three of the five also carry a `main` so the counts
behind the inequalities can be printed rather than only bounded.

Two probes refused on their first compile, both times because a claim of mine was wrong rather than
because the code was. Both corrections are recorded in the probe headers and in file 33, because a
check that only ever passes is not evidence that it is checking anything.

| Probe | Build | What it establishes |
|---|---|---|
| `probe_1_monotone_is_distributivity.rs` | clean, 0.8s | On a totally ordered value set, monotonicity in each argument and distributivity over `max` and over `min` are the same fact, checked as an equivalence with a witnessed failing side. Reproduces the associativity/distributivity inversion between wrapping and saturating addition independently. Shows neither is a dioid over `(max, op)`, and that saturating addition fails the dioid axioms twice over, so repairing associativity alone would not produce one. |
| `probe_2_interior_safety_upgrades_weak_to_kleene.rs` | clean, ~90s const eval | Over all 14 groupings of a five-element fold and all 8^5 inputs, at four accumulator ranges: zero value disagreements at every width, and definedness splits that go 216, 36, 0, 0 as the accumulator widens. Interior safety is the condition that upgrades the weak equation to the Kleene equation. The closed form `(n-1) * [min V(N), max V(N)] subset V(M)` is sufficient and measurably not necessary. |
| `probe_3_product_numeral_is_associative.rs` | clean, 3.5s | File 31's product-numeral formula is commutative and associative, over every ordered triple from a 6x5 grid of (adjustment, bias) pairs, and both bracketings equal a direct ternary closed form: bias is the all-B monomial, adjustment is the gcd of the seven monomials carrying at least one A. Containment checked on three triples; the naive adjustment without cross terms fails containment. |
| `probe_4_four_atoms_beat_three_class_names.rs` | clean, 1.8s | Four atomic properties (total, fixes, monotone, homomorphic) give five distinct signatures for the design's five recovery maps. The three declared class names place three of the five and have nowhere to put `SubstituteZero` or the confined dithered entry point. |
| `probe_5_direction_enters_the_key_iff_the_lattice_opens.rs` | clean, 0.9s | The four `Direction` instances agree at every operand pair for addition and for `mul_full`, and disagree on 128 of 256 pairs for narrowed multiplication. Additive lattice closure is `bias/adjustment` integral; narrowed-multiplicative closure additionally needs the adjustment itself integral, which no fractional fixed-point numeral satisfies. Every `Direction` is monotone; a wrapping resolution is not. |

## Reproduction

```
cd mock/research/202607301300_formalization-spec-panel/33_probes
for f in probe_*.rs; do rustc --edition 2021 --crate-type lib "$f" -o /tmp/$(basename $f .rs).rlib; done
# and, for the three carrying a main:
rustc --edition 2021 probe_2_interior_safety_upgrades_weak_to_kleene.rs -o /tmp/p2 && /tmp/p2
rustc --edition 2021 probe_3_product_numeral_is_associative.rs -o /tmp/p3 && /tmp/p3
rustc --edition 2021 probe_4_four_atoms_beat_three_class_names.rs -o /tmp/p4 && /tmp/p4
rustc --edition 2021 probe_5_direction_enters_the_key_iff_the_lattice_opens.rs -o /tmp/p5 && /tmp/p5
```

Probe 2 carries `#![allow(long_running_const_eval)]`. Its check is 32768 inputs by 14 groupings by
four accumulator ranges, which is the same const-eval wall `26_consolidation_two.md:72-74` records;
every loop bound in it is a literal, so it terminates by construction.

## Printed output

```
$ /tmp/p2
acc_range     value_disagreements  definedness_splits  defined_under_all
[ -8,  7]          0                  216               16920
[ -9,  8]          0                   36               17100
[-10,  9]          0                    0               17136
[-16, 12]          0                    0               17136
closed-form sufficient bound: [-16, 12]
exact_fits_count = 17136

$ /tmp/p3
mulnum3(T1,T2,T3) = adjustment 4 bias 40
naive               adjustment 72 bias 40
all-bias-zero       adjustment 72 bias 0

$ /tmp/p4
map              T F M H  classified
ReduceModulo     1 1 0 1  1
Clamp            1 1 1 0  1
Refuse           0 1 1 0  1
SubstituteZero   1 1 0 0  0
DitherConfined   1 0 1 0  0

$ /tmp/p5
narrowed-multiply direction disagreements: 128 of 256 operand pairs
addition: all four directions agree at every pair
mul_full: all four directions agree at every pair
```

## The two refusals, kept rather than smoothed over

**Probe 2, first draft.** I stated interior safety as `n * [min V(N), max V(N)] subset V(M)` and
predicted a definedness split at an accumulator that shows none. The const assertion refused. The
sufficient bound is `(n-1)`, not `n`, because every proper subtree of a fold over `n` operands holds
a sum of at most `n-1` of them and the root's argument is grouping-independent once they are all
exact. `26_consolidation_two.md:717` already says `K = n - 1` in its droplist; I had derived `n`
independently and the compiler, not the citation, is what caught it. The rewritten probe then
measured that even `(n-1)` is conservative: an accumulator of `[-10, 9]` against a closed-form bound
of `[-16, 12]` already shows no split, because the destination numeral's own range prunes the inputs
that could produce one.

**Probe 4, first draft.** I quantified the homomorphism condition over the destination numeral rather
than over the exact domain, which makes it vacuously true for every map that fixes the destination
pointwise, and four of the five do. It reported clamping as a homomorphism. The const assertion
refused. Quantifying over the exact domain, where the maps actually differ from the identity, gives
the table above.
