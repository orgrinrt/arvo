# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 385% faster than the next best (quantiser-software)

quantiser-hardware (920 ns) leads quantiser-software (4.46 us) by 385%, a clear separation rather than a photo finish. CV 23.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (920 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest quantiser-hardware (920 ns) to slowest quantiser-software (4.46 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 2.9x its best-20%

quantiser-software's best 20% of batches run at 3.63 us but its worst 20% at 10.68 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 919.9 ns median
- 1 variant significantly slower than baseline
- Spread: 4.85x (fastest 919.9 ns, slowest 4456.7 ns)

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
| quantiser-hardware | 1197ns | 1242ns | 854ns | 1162ns | 1644ns | base |
| quantiser-software | 5627ns | 4526ns | 3685ns | 4505ns | 10934ns | +370.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 875ns | 625ns | 1199ns | base | 0.292 |
| quantiser-software | 5523ns | 3628ns | 10680ns | +530.91% | 0.046 |

## Performance model

- Peak throughput: **0.410 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.278 | 67.9% |
| quantiser-software | 0.057 | 14.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1197ns | 1197ns | base |
| quantiser-software | 5627ns | 5627ns | +370.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 920ns | base | --- | [707, 956] | --- | --- | --- | --- |
| quantiser-software | 4457ns | +3776.1ns (+410.5%) | [+3503, +3824]ns | [4447, 4501] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 700ns | +545.3% |
| 1 | 2 | 1037ns | +329.6% |
| 1 | 3 | 998ns | +263.8% |
| 1 | 4 | 660ns | +575.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.243 | moderate- |
| quantiser-software | -0.362 | moderate- |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 108.2ns | 875.4ns | 12.4% | HIGH |
| quantiser-software | 29.6ns | 5523.1ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 624.7-1199.1 ns)
    624.7 |#####
    653.4 |########################################
    682.1 |#################
    710.8 |###########
    739.6 |#####
    768.3 |###########
    797.0 |
    825.7 |
    854.5 |
    883.2 |
    911.9 |#################
    940.6 |##################################
    969.3 |#####
    998.1 |#####
   1026.8 |###########
   1055.5 |
   1084.2 |
   1112.9 |#################
   1141.7 |###########
   1170.4 |#####
  (3 below, 2 above range)

quantiser-software (n=40, range 3628.2-10679.9 ns)
   3628.2 |######
   3980.8 |
   4333.4 |########################################
   4686.0 |
   5038.6 |
   5391.1 |
   5743.7 |
   6096.3 |
   6448.9 |
   6801.5 |
   7154.1 |
   7506.6 |
   7859.2 |
   8211.8 |
   8564.4 |#
   8917.0 |
   9269.5 |
   9622.1 |
   9974.7 |
  10327.3 |#
  (5 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=24.9% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=11.7% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=57.5% (high variance, measurements may be unstable)
