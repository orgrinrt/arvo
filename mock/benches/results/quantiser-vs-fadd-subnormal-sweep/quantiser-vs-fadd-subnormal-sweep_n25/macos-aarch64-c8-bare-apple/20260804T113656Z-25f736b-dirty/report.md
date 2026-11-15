# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1477% faster than the next best (quantiser-software)

quantiser-hardware (318 ns) leads quantiser-software (5.02 us) by 1477%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows warm-up / thermal drift (autocorr +0.79)

quantiser-hardware's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (318 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.8x the fastest

Fastest quantiser-hardware (318 ns) to slowest quantiser-software (5.02 us): 15.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 318.1 ns median
- 1 variant significantly slower than baseline
- Spread: 15.77x (fastest 318.1 ns, slowest 5015.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 392ns | 391ns | 374ns | 392ns | 410ns | base |
| quantiser-software | 5053ns | 5098ns | 4753ns | 5030ns | 5418ns | +1188.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 318ns | 304ns | 331ns | base | 0.806 |
| quantiser-software | 4968ns | 4675ns | 5314ns | +1463.71% | 0.052 |

## Performance model

- Peak throughput: **0.843 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.805 | 95.4% |
| quantiser-software | 0.051 | 6.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 392ns | 392ns | base |
| quantiser-software | 5053ns | 5053ns | +1188.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 318ns | base | --- | [305, 330] | --- | --- | --- | --- |
| quantiser-software | 5016ns | +4685.4ns (+1472.7%) | [+4459, +4739]ns | [4764, 5062] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 332ns | +1412.3% |
| 2 | 330ns | +1420.2% |
| 3 | 331ns | +1417.0% |
| 4 | 328ns | +1432.2% |
| 5 | 331ns | +1415.8% |
| 6 | 329ns | +1423.6% |
| 7 | 332ns | +1409.2% |
| 8 | 330ns | +1421.8% |
| 9 | 330ns | +1417.9% |
| 10 | 331ns | +1423.3% |
| 11 | 305ns | +1582.2% |
| 12 | 304ns | +1596.6% |
| 13 | 309ns | +1546.6% |
| 14 | 308ns | +1444.8% |
| 15 | 305ns | +1463.4% |
| 16 | 305ns | +1460.2% |
| 17 | 303ns | +1474.6% |
| 18 | 304ns | +1454.9% |
| 19 | 302ns | +1464.2% |
| 20 | 305ns | +1451.6% |
| 21 | 330ns | +1504.4% |
| 22 | 328ns | +1501.9% |
| 23 | 330ns | +1447.6% |
| 24 | 332ns | +1437.9% |
| 25 | 331ns | +1462.9% |
| 26 | 330ns | +1610.4% |
| 27 | 331ns | +1522.1% |
| 28 | 331ns | +1443.0% |
| 29 | 330ns | +1450.1% |
| 30 | 331ns | +1445.0% |
| 31 | 302ns | +1458.9% |
| 32 | 305ns | +1440.3% |
| 33 | 305ns | +1453.4% |
| 34 | 305ns | +1429.1% |
| 35 | 306ns | +1427.3% |
| 36 | 308ns | +1416.3% |
| 37 | 304ns | +1430.9% |
| 38 | 307ns | +1408.5% |
| 39 | 304ns | +1444.0% |
| 40 | 304ns | +1704.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.795 | HIGH+ (drift/warm-up) |
| quantiser-software | 0.602 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.8ns | 317.7ns | 0.9% |  |
| quantiser-software | 2.8ns | 4968.0ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 303.5-331.3 ns)
    303.5 |#######################
    304.9 |####################
    306.3 |###
    307.7 |##########
    309.1 |
    310.5 |
    311.9 |
    313.3 |
    314.6 |
    316.0 |
    317.4 |
    318.8 |
    320.2 |
    321.6 |
    323.0 |
    324.4 |
    325.8 |
    327.2 |######
    328.6 |##########
    329.9 |########################################
  (3 below, 3 above range)

quantiser-software (n=40, range 4675.4-5314.5 ns)
   4675.4 |########
   4707.4 |#################
   4739.3 |#################
   4771.3 |####
   4803.2 |
   4835.2 |
   4867.1 |
   4899.1 |
   4931.0 |
   4963.0 |
   4994.9 |########################################
   5026.9 |####
   5058.9 |####
   5090.8 |######################
   5122.8 |####
   5154.7 |########
   5186.7 |
   5218.6 |
   5250.6 |####
   5282.5 |####
  (5 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **quantiser-software**: autocorrelation=0.60 (measurement drift or warm-up artifact)
