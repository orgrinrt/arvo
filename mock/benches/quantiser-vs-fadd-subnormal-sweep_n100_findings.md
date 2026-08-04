# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1141% faster than the next best (quantiser-software)

quantiser-hardware (286 ns) leads quantiser-software (3.55 us) by 1141%, a clear separation rather than a photo finish. CV 16.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware is fastest but the noisiest (CV 16.5%)

quantiser-hardware wins on median (286 ns) yet has the highest variance (CV 16.5%), while quantiser-software is the steadiest (CV 4.1%, 3.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-software shows warm-up / thermal drift (autocorr +0.73)

quantiser-software's per-pass series has lag-1 autocorrelation +0.73, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (286 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 12.4x the fastest

Fastest quantiser-hardware (286 ns) to slowest quantiser-software (3.55 us): 12.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 286.4 ns median
- 1 variant significantly slower than baseline
- Spread: 12.41x (fastest 286.4 ns, slowest 3554.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 365ns | 354ns | 334ns | 357ns | 421ns | base |
| quantiser-software | 3693ns | 3622ns | 3539ns | 3669ns | 3918ns | +911.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 298ns | 271ns | 350ns | base | 0.860 |
| quantiser-software | 3621ns | 3470ns | 3838ns | +1116.53% | 0.071 |

## Performance model

- Peak throughput: **0.946 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.894 | 94.5% |
| quantiser-software | 0.072 | 7.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 365ns | 365ns | base |
| quantiser-software | 3693ns | 3693ns | +911.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 286ns | base | --- | [285, 288] | --- | --- | --- | --- |
| quantiser-software | 3555ns | +3268.8ns (+1141.1%) | [+3257, +3428]ns | [3524, 3723] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 306ns | +1116.9% |
| 2 | 308ns | +1134.7% |
| 3 | 305ns | +1039.9% |
| 4 | 304ns | +1041.6% |
| 5 | 305ns | +1037.4% |
| 6 | 307ns | +1031.7% |
| 7 | 307ns | +1032.9% |
| 8 | 306ns | +1034.2% |
| 9 | 303ns | +1043.8% |
| 10 | 305ns | +1036.5% |
| 11 | 265ns | +1230.8% |
| 12 | 267ns | +1221.2% |
| 13 | 266ns | +1225.1% |
| 14 | 264ns | +1236.5% |
| 15 | 261ns | +1249.0% |
| 16 | 276ns | +1175.8% |
| 17 | 286ns | +1131.5% |
| 18 | 285ns | +1135.7% |
| 19 | 287ns | +1129.4% |
| 20 | 285ns | +1134.9% |
| 21 | 285ns | +1145.3% |
| 22 | 288ns | +1135.9% |
| 23 | 287ns | +1140.1% |
| 24 | 282ns | +1158.7% |
| 25 | 285ns | +1148.4% |
| 26 | 287ns | +1139.2% |
| 27 | 288ns | +1210.4% |
| 28 | 285ns | +1230.7% |
| 29 | 561ns | +627.7% |
| 30 | 290ns | +1209.1% |
| 31 | 286ns | +1202.3% |
| 32 | 285ns | +1206.7% |
| 33 | 285ns | +1205.0% |
| 34 | 287ns | +1199.3% |
| 35 | 284ns | +1231.2% |
| 36 | 285ns | +1204.4% |
| 37 | 286ns | +1204.0% |
| 38 | 288ns | +1267.5% |
| 39 | 284ns | +1210.1% |
| 40 | 399ns | +833.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.019 | ok |
| quantiser-software | 0.730 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.6ns | 297.6ns | 0.9% |  |
| quantiser-software | 2.1ns | 3620.8ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 270.7-349.9 ns)
    270.7 |
    274.6 |###
    278.6 |###
    282.6 |########################################
    286.5 |########################
    290.5 |
    294.4 |
    298.4 |
    302.4 |#####################
    306.3 |#########
    310.3 |
    314.2 |
    318.2 |
    322.2 |
    326.1 |
    330.1 |
    334.0 |
    338.0 |
    342.0 |
    345.9 |
  (5 below, 2 above range)

quantiser-software (n=40, range 3470.4-3837.8 ns)
   3470.4 |#################
   3488.8 |
   3507.1 |###################################
   3525.5 |########
   3543.9 |##########################
   3562.2 |
   3580.6 |
   3599.0 |
   3617.3 |
   3635.7 |
   3654.1 |
   3672.4 |
   3690.8 |
   3709.2 |########################################
   3727.5 |
   3745.9 |
   3764.3 |####
   3782.6 |#############
   3801.0 |####
   3819.4 |
  (4 below, 2 above range)

```

## Diagnostics

- **quantiser-software**: autocorrelation=0.73 (measurement drift or warm-up artifact)
