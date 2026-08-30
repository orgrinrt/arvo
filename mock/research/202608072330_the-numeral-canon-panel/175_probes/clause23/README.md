# P4: do clauses 2 and 3 denote together

```sh
python3 clause23.py > clause23.out
```

Clause 2 licenses "any realisation that induces **the stretch's boundary function**". Clause 3 says an
adaptation point on an unbound edge "is free and **placed under clause 2**", and in the same clause
that "two schedules over the same operations compute different functions".

If both hold, a stretch with a free adaptation point has a **family** of boundary functions, so the
definite description in clause 2 has no unique referent there.

## Result: the antecedent is real

Distinct boundary functions over all interior placements, inputs exhaustive over 0..=255:

| chain | resolution = round-to-8 | resolution = identity (C-A) |
|---|---|---|
| `+97 *3 +13` | 1 | 1 |
| `*3 >>1 *5` | **4** | 1 |
| `+97 *5 >>1 +13` | **3** | 1 |
| `*3 *3 +97` | **3** | 1 |

Three of four chains have more than one. The witness `*3 >>1 *5` has four, and two of its placements
differ at the **boundary** on 30 of 256 inputs.

## Two of my own controls failed first, and both runs are kept

`clause23_v1_CONTROLS_FAILED.out` is that run.

**C-A failed**, reporting 2 distinct functions under what I had called the identity resolution. My
"identity" was `clamp`, which is not the identity on intermediates exceeding 255, and every chain here
produces some. **A control has to be the identity on the reachable set, not on the declared one.**

**C-D failed**, reading 0 of 256, because I ran it on `chains[0]`, which has one boundary function. The
control must run on a chain the antecedent actually holds for; it now picks the witness.

Both were my defects rather than findings, and both are the same class: a control evaluated somewhere
the phenomenon is absent reports zero and reads as a refutation.

`holds for: W = 8, resolution = round-to-nearest onto the multiples of 8 with a clamp at 248, ops in
{+97, +13, *3, *5, >>1}, chains of depth 3 and 4, all 2^(depth-1) interior placements, inputs
exhaustive over 0..=255, threads = 1`
