# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 553% faster than the next best (quantiser-software)

quantiser-hardware (703 ns) leads quantiser-software (4.59 us) by 553%, a clear separation rather than a photo finish. CV 67.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware is fastest but the noisiest (CV 67.8%)

quantiser-hardware wins on median (703 ns) yet has the highest variance (CV 67.8%), while quantiser-software is the steadiest (CV 67.1%, 4.59 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (703 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.5x the fastest

Fastest quantiser-hardware (703 ns) to slowest quantiser-software (4.59 us): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-hardware is inconsistent: worst-20% is 3.0x its best-20%

quantiser-hardware's best 20% of batches run at 531 ns but its worst 20% at 1.58 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 703.5 ns median
- 1 variant significantly slower than baseline
- Spread: 6.53x (fastest 703.5 ns, slowest 4594.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1267ns | 987ns | 744ns | 1166ns | 2093ns | base |
| quantiser-software | 5544ns | 4662ns | 3812ns | 4448ns | 10561ns | +337.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 920ns | 531ns | 1576ns | base | 0.278 |
| quantiser-software | 5447ns | 3756ns | 10327ns | +492.03% | 0.047 |

## Performance model

- Peak throughput: **0.482 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.364 | 75.5% |
| quantiser-software | 0.056 | 11.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1267ns | 1267ns | base |
| quantiser-software | 5544ns | 5544ns | +337.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 703ns | base | --- | [674, 983] | --- | --- | --- | --- |
| quantiser-software | 4594ns | +3471.8ns (+493.5%) | [+3288, +3881]ns | [3789, 4670] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 621ns | +652.5% |
| 1 | 2 | 1124ns | +308.8% |
| 1 | 3 | 748ns | +402.3% |
| 1 | 4 | 703ns | +437.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.466 | moderate- |
| quantiser-software | 0.264 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 117.8ns | 920.0ns | 12.8% | HIGH |
| quantiser-software | 28.4ns | 5446.6ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 531.4-1576.1 ns)
    531.4 |
    583.6 |###################################
    635.8 |####
    688.1 |###############################
    740.3 |
    792.5 |####
    844.8 |
    897.0 |####
    949.2 |#################
   1001.5 |
   1053.7 |
   1106.0 |########################################
   1158.2 |
   1210.4 |####
   1262.7 |
   1314.9 |
   1367.2 |
   1419.4 |
   1471.6 |
   1523.9 |
  (5 below, 3 above range)

quantiser-software (n=40, range 3756.4-10327.4 ns)
   3756.4 |#######################
   4085.0 |
   4413.5 |########################################
   4742.1 |
   5070.6 |
   5399.2 |
   5727.7 |
   6056.3 |
   6384.8 |##
   6713.4 |
   7041.9 |
   7370.5 |
   7699.0 |
   8027.6 |
   8356.1 |
   8684.6 |
   9013.2 |
   9341.7 |
   9670.3 |
   9998.8 |
  (4 below, 5 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=51.8% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=15.2% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=56.6% (high variance, measurements may be unstable)
