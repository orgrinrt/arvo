# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 437% faster than the next best (quantiser-software)

quantiser-hardware (829 ns) leads quantiser-software (4.45 us) by 437%, a clear separation rather than a photo finish. CV 13.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (829 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.4x the fastest

Fastest quantiser-hardware (829 ns) to slowest quantiser-software (4.45 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.5x its best-20%

quantiser-software's best 20% of batches run at 3.67 us but its worst 20% at 12.81 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 828.7 ns median
- 1 variant significantly slower than baseline
- Spread: 5.37x (fastest 828.7 ns, slowest 4449.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1165ns | 1133ns | 988ns | 1140ns | 1418ns | base |
| quantiser-software | 6083ns | 4518ns | 3725ns | 4532ns | 13094ns | +422.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 848ns | 725ns | 1032ns | base | 0.302 |
| quantiser-software | 5973ns | 3669ns | 12807ns | +604.10% | 0.043 |

## Performance model

- Peak throughput: **0.353 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.309 | 87.5% |
| quantiser-software | 0.058 | 16.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1165ns | 1165ns | base |
| quantiser-software | 6083ns | 6083ns | +422.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 829ns | base | --- | [758, 871] | --- | --- | --- | --- |
| quantiser-software | 4449ns | +3707.0ns (+447.3%) | [+2926, +3738]ns | [3711, 4466] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 962ns | +285.6% |
| 1 | 2 | 849ns | +332.8% |
| 1 | 3 | 749ns | +494.1% |
| 1 | 4 | 735ns | +508.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.248 | moderate+ |
| quantiser-software | 0.245 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 105.2ns | 848.3ns | 12.4% | HIGH |
| quantiser-software | 31.8ns | 5972.8ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 724.8-1031.9 ns)
    724.8 |########################################
    740.2 |#####
    755.5 |##########
    770.9 |
    786.2 |#####
    801.6 |
    817.0 |#########################
    832.3 |###############
    847.7 |
    863.0 |###############
    878.4 |
    893.7 |###############
    909.1 |
    924.5 |#####
    939.8 |
    955.2 |###############
    970.5 |#####
    985.9 |
   1001.2 |
   1016.6 |##########
  (4 below, 3 above range)

quantiser-software (n=40, range 3668.9-12806.9 ns)
   3668.9 |###############################
   4125.8 |########################################
   4582.7 |
   5039.6 |##
   5496.5 |##
   5953.4 |
   6410.3 |
   6867.2 |
   7324.1 |
   7781.0 |##
   8237.9 |
   8694.8 |
   9151.7 |
   9608.6 |
  10065.5 |
  10522.4 |
  10979.3 |##
  11436.2 |##
  11893.1 |
  12350.0 |#####
  (4 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: bridge=12.7% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=59.0% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.5x (possible bimodal distribution)
