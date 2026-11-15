# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 498% faster than the next best (quantiser-software)

quantiser-hardware (710 ns) leads quantiser-software (4.25 us) by 498%, a clear separation rather than a photo finish. CV 22.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (710 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.0x the fastest

Fastest quantiser-hardware (710 ns) to slowest quantiser-software (4.25 us): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.3x its best-20%

quantiser-software's best 20% of batches run at 3.60 us but its worst 20% at 11.93 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 709.5 ns median
- 1 variant significantly slower than baseline
- Spread: 5.98x (fastest 709.5 ns, slowest 4246.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 990ns | 1009ns | 725ns | 981ns | 1281ns | base |
| quantiser-software | 5804ns | 4317ns | 3655ns | 4392ns | 12188ns | +486.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 714ns | 524ns | 916ns | base | 0.359 |
| quantiser-software | 5698ns | 3597ns | 11928ns | +698.14% | 0.045 |

## Performance model

- Peak throughput: **0.489 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.361 | 73.8% |
| quantiser-software | 0.060 | 12.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 990ns | 990ns | base |
| quantiser-software | 5804ns | 5804ns | +486.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 710ns | base | --- | [608, 834] | --- | --- | --- | --- |
| quantiser-software | 4246ns | +3556.0ns (+501.2%) | [+3435, +3667]ns | [4233, 4321] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 840ns | +415.3% |
| 1 | 2 | 836ns | +407.7% |
| 1 | 3 | 558ns | +658.9% |
| 1 | 4 | 586ns | +498.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.223 | moderate+ |
| quantiser-software | -0.046 | ok |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 89.9ns | 713.9ns | 12.6% | HIGH |
| quantiser-software | 31.5ns | 5697.6ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 523.6-915.8 ns)
    523.6 |
    543.3 |########################################
    562.9 |
    582.5 |#####
    602.1 |
    621.7 |##########
    641.3 |#####
    660.9 |##########
    680.5 |
    700.1 |##########
    719.7 |
    739.4 |#####
    759.0 |
    778.6 |#####
    798.2 |##########
    817.8 |##########
    837.4 |###################################
    857.0 |
    876.6 |####################
    896.2 |#####
  (5 below, 1 above range)

quantiser-software (n=40, range 3596.8-11928.3 ns)
   3596.8 |
   4013.4 |########################################
   4430.0 |
   4846.6 |
   5263.1 |#
   5679.7 |
   6096.3 |
   6512.8 |
   6929.4 |
   7346.0 |
   7762.5 |
   8179.1 |
   8595.7 |#
   9012.3 |###
   9428.8 |
   9845.4 |
  10262.0 |
  10678.5 |
  11095.1 |#
  11511.7 |
  (7 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=22.1% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=12.7% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=58.7% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.3x (possible bimodal distribution)
