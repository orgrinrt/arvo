# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 224% faster than the next best (quantiser-software)

quantiser-hardware (1.15 us) leads quantiser-software (3.74 us) by 224%, a clear separation rather than a photo finish. CV 21.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (1.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest quantiser-hardware (1.15 us) to slowest quantiser-software (3.74 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.5x its best-20%

quantiser-software's best 20% of batches run at 3.02 us but its worst 20% at 10.58 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 1153.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.24x (fastest 1153.4 ns, slowest 3741.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Round-first software quantiser add vs native hardware fadd, subnormal fraction swept |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112891188 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1541ns | 1453ns | 1208ns | 1496ns | 2008ns | base |
| quantiser-software | 5397ns | 3811ns | 3083ns | 4346ns | 10863ns | +250.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 1214ns | 911ns | 1581ns | base | 0.211 |
| quantiser-software | 5281ns | 3025ns | 10581ns | +335.07% | 0.048 |

## Performance model

- Peak throughput: **0.281 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.222 | 78.9% |
| quantiser-software | 0.068 | 24.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1541ns | 1541ns | base |
| quantiser-software | 5397ns | 5397ns | +250.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 1153ns | base | --- | [1123, 1240] | --- | --- | --- | --- |
| quantiser-software | 3741ns | +2581.1ns (+223.8%) | [+2313, +2673]ns | [3699, 3787] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 1160ns | +162.8% |
| 1 | 2 | 1110ns | +172.9% |
| 1 | 3 | 1377ns | +168.9% |
| 1 | 4 | 1153ns | +228.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.478 | moderate- |
| quantiser-software | 0.267 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 109.7ns | 1213.8ns | 9.0% | HIGH |
| quantiser-software | 34.0ns | 5280.9ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 910.5-1581.5 ns)
    910.5 |
    944.1 |
    977.6 |########
   1011.2 |########
   1044.7 |########################
   1078.3 |########################
   1111.8 |########################################
   1145.3 |########################################
   1178.9 |########################
   1212.4 |########
   1246.0 |########
   1279.5 |
   1313.1 |########
   1346.6 |################
   1380.2 |################
   1413.7 |########
   1447.3 |########
   1480.8 |################
   1514.4 |
   1547.9 |########
  (4 below, 3 above range)

quantiser-software (n=40, range 3025.0-10580.7 ns)
   3025.0 |##########################
   3402.8 |########################################
   3780.6 |##########
   4158.3 |
   4536.1 |
   4913.9 |######
   5291.7 |
   5669.5 |
   6047.3 |###
   6425.1 |
   6802.9 |
   7180.6 |
   7558.4 |
   7936.2 |
   8314.0 |###
   8691.8 |######
   9069.6 |######
   9447.4 |###
   9825.2 |
  10202.9 |###
  (4 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=20.4% (high variance, measurements may be unstable)
- **quantiser-software**: CV=57.1% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.5x (possible bimodal distribution)
