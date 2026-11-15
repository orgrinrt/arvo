# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1332% faster than the next best (quantiser-software)

quantiser-hardware (330 ns) leads quantiser-software (4.72 us) by 1332%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows warm-up / thermal drift (autocorr +0.89)

quantiser-hardware's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (330 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 14.3x the fastest

Fastest quantiser-hardware (330 ns) to slowest quantiser-software (4.72 us): 14.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 330.0 ns median
- 1 variant significantly slower than baseline
- Spread: 14.32x (fastest 330.0 ns, slowest 4724.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 400ns | 409ns | 375ns | 405ns | 412ns | base |
| quantiser-software | 4784ns | 4803ns | 4398ns | 4773ns | 5202ns | +1094.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 324ns | 304ns | 332ns | base | 0.790 |
| quantiser-software | 4693ns | 4319ns | 5065ns | +1347.81% | 0.055 |

## Performance model

- Peak throughput: **0.842 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.776 | 92.2% |
| quantiser-software | 0.054 | 6.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 400ns | 400ns | base |
| quantiser-software | 4784ns | 4784ns | +1094.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 330ns | base | --- | [329, 330] | --- | --- | --- | --- |
| quantiser-software | 4725ns | +4394.4ns (+1331.6%) | [+4298, +4402]ns | [4629, 4731] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 305ns | +1425.5% |
| 2 | 305ns | +1395.0% |
| 3 | 303ns | +1312.7% |
| 4 | 303ns | +1483.6% |
| 5 | 304ns | +1309.3% |
| 6 | 305ns | +1302.8% |
| 7 | 306ns | +1299.5% |
| 8 | 306ns | +1299.5% |
| 9 | 303ns | +1313.2% |
| 10 | 305ns | +1304.9% |
| 11 | 330ns | +1335.5% |
| 12 | 330ns | +1338.0% |
| 13 | 328ns | +1343.0% |
| 14 | 330ns | +1338.3% |
| 15 | 329ns | +1340.4% |
| 16 | 330ns | +1333.9% |
| 17 | 330ns | +1331.5% |
| 18 | 331ns | +1327.5% |
| 19 | 330ns | +1330.7% |
| 20 | 330ns | +1336.9% |
| 21 | 332ns | +1393.3% |
| 22 | 332ns | +1554.8% |
| 23 | 332ns | +1324.2% |
| 24 | 330ns | +1332.5% |
| 25 | 331ns | +1472.2% |
| 26 | 332ns | +1323.6% |
| 27 | 330ns | +1348.8% |
| 28 | 331ns | +1327.1% |
| 29 | 330ns | +1330.2% |
| 30 | 333ns | +1437.3% |
| 31 | 330ns | +1375.4% |
| 32 | 329ns | +1303.6% |
| 33 | 329ns | +1305.9% |
| 34 | 330ns | +1301.8% |
| 35 | 333ns | +1290.6% |
| 36 | 331ns | +1299.1% |
| 37 | 332ns | +1292.7% |
| 38 | 330ns | +1301.0% |
| 39 | 330ns | +1498.0% |
| 40 | 331ns | +1295.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.889 | HIGH+ (drift/warm-up) |
| quantiser-software | 0.400 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.3ns | 324.1ns | 0.7% |  |
| quantiser-software | 2.4ns | 4692.9ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 304.2-332.3 ns)
    304.2 |#########
    305.6 |####
    307.0 |
    308.4 |
    309.8 |
    311.2 |
    312.6 |
    314.0 |
    315.4 |
    316.8 |
    318.2 |
    319.7 |
    321.1 |
    322.5 |
    323.9 |
    325.3 |
    326.7 |
    328.1 |#########
    329.5 |########################################
    330.9 |###########
  (4 below, 4 above range)

quantiser-software (n=40, range 4319.0-5064.7 ns)
   4319.0 |
   4356.2 |
   4393.5 |
   4430.8 |
   4468.1 |
   4505.4 |
   4542.7 |####
   4580.0 |####
   4617.3 |###################################
   4654.5 |
   4691.8 |########################################
   4729.1 |##########################
   4766.4 |########
   4803.7 |
   4841.0 |####
   4878.3 |
   4915.6 |
   4952.8 |####
   4990.1 |
   5027.4 |
  (7 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: autocorrelation=0.89 (measurement drift or warm-up artifact)
