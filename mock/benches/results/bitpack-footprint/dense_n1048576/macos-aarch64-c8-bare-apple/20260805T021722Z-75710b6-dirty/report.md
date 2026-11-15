# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (116.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 113.12 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 113122.7 ns median (-2.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 113122.7 ns, slowest 116163.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 117813ns | 116534ns | 110729ns | 116109ns | 130009ns | base |
| bitpack-footprint-dense-alt | 114434ns | 113372ns | 110165ns | 114584ns | 118254ns | -2.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 117470ns | 110431ns | 129576ns | base | 8.926 |
| bitpack-footprint-dense-alt | 114111ns | 109832ns | 117913ns | -2.86% | 9.189 |

## Performance model

- Peak throughput: **9.547 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 9.027 | 94.5% |
| bitpack-footprint-dense-alt | 9.269 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 117813ns | 117813ns | base |
| bitpack-footprint-dense-alt | 114434ns | 114434ns | -2.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 116164ns | base | --- | [114654, 117249] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 113123ns | -1965.2ns (-1.7%) | [-3532, -341]ns | [112892, 116003] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|
| 1 | 116874ns | -6.2% |
| 2 | 116845ns | -0.6% |
| 3 | 117360ns | -2.1% |
| 4 | 116381ns | -1.5% |
| 5 | 181912ns | -35.8% |
| 6 | 134014ns | -12.8% |
| 7 | 127631ns | -8.4% |
| 8 | 119491ns | -2.2% |
| 9 | 115280ns | +3.4% |
| 10 | 117553ns | +1.5% |
| 11 | 117739ns | -3.9% |
| 12 | 117312ns | +0.3% |
| 13 | 113304ns | -0.5% |
| 14 | 113340ns | -0.4% |
| 15 | 117393ns | +0.2% |
| 16 | 117185ns | -1.1% |
| 17 | 117618ns | -4.6% |
| 18 | 118998ns | -9.2% |
| 19 | 115196ns | -2.5% |
| 20 | 115879ns | -2.5% |
| 21 | 116440ns | -1.7% |
| 22 | 111765ns | +4.7% |
| 23 | 112082ns | +0.8% |
| 24 | 108344ns | +4.3% |
| 25 | 111367ns | +5.1% |
| 26 | 108534ns | +4.2% |
| 27 | 110445ns | +1.6% |
| 28 | 113151ns | -4.2% |
| 29 | 113675ns | -5.3% |
| 30 | 114568ns | -5.7% |
| 31 | 115389ns | +1.5% |
| 32 | 112889ns | -0.2% |
| 33 | 115946ns | -2.8% |
| 34 | 114740ns | -1.7% |
| 35 | 117350ns | -4.0% |
| 36 | 118304ns | -3.2% |
| 37 | 118522ns | -4.8% |
| 38 | 112633ns | +4.3% |
| 39 | 117067ns | -3.5% |
| 40 | 108280ns | +8.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.304 | moderate+ |
| bitpack-footprint-dense-alt | 0.327 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 27/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 13.8ns | 117469.9ns | 0.0% |  |
| bitpack-footprint-dense-alt | 3.6ns | 114110.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 110431.1-129576.2 ns)
  110431.1 |##########
  111388.4 |##########
  112345.6 |###############
  113302.9 |###############
  114260.1 |###############
  115217.4 |####################
  116174.6 |#########################
  117131.9 |########################################
  118089.2 |###############
  119046.4 |#####
  120003.7 |
  120960.9 |
  121918.2 |
  122875.4 |
  123832.7 |
  124790.0 |
  125747.2 |
  126704.5 |#####
  127661.7 |
  128619.0 |
  (3 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 109832.1-117912.6 ns)
  109832.1 |
  110236.1 |
  110640.1 |
  111044.2 |
  111448.2 |
  111852.2 |##########
  112256.2 |####################
  112660.3 |########################################
  113064.3 |##########
  113468.3 |
  113872.3 |
  114276.4 |###############
  114680.4 |#####
  115084.4 |
  115488.4 |#####
  115892.5 |#####
  116296.5 |
  116700.5 |###################################
  117104.5 |#####
  117508.6 |##########
  (5 below, 3 above range)

```
