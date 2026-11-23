# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 370% faster than the next best (quantiser-software)

quantiser-hardware (798 ns) leads quantiser-software (3.75 us) by 370%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (798 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.7x the fastest

Fastest quantiser-hardware (798 ns) to slowest quantiser-software (3.75 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.0x its best-20%

quantiser-software's best 20% of batches run at 3.73 us but its worst 20% at 11.07 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 798.1 ns median
- 1 variant significantly slower than baseline
- Spread: 4.70x (fastest 798.1 ns, slowest 3753.4 ns)

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
| Counter frequency | 2112817978 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1134ns | 1122ns | 952ns | 1107ns | 1394ns | base |
| quantiser-software | 5303ns | 3813ns | 3782ns | 3816ns | 11284ns | +367.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 780ns | 677ns | 913ns | base | 0.328 |
| quantiser-software | 5214ns | 3725ns | 11066ns | +568.37% | 0.049 |

## Performance model

- Peak throughput: **0.378 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.321 | 84.8% |
| quantiser-software | 0.068 | 18.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1134ns | 1134ns | base |
| quantiser-software | 5303ns | 5303ns | +367.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 798ns | base | --- | [704, 800] | --- | --- | --- | --- |
| quantiser-software | 3753ns | +3031.8ns (+379.9%) | [+2975, +3053]ns | [3741, 3771] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 798ns | +374.6% |
| 1 | 2 | 933ns | +299.5% |
| 1 | 3 | 700ns | +436.1% |
| 1 | 4 | 742ns | +403.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.244 | moderate- |
| quantiser-software | -0.445 | moderate- |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 118.1ns | 780.1ns | 15.1% | HIGH |
| quantiser-software | 27.3ns | 5213.7ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 676.8-912.9 ns)
    676.8 |
    688.6 |####
    700.4 |#######################
    712.2 |
    724.0 |
    735.8 |
    747.6 |
    759.4 |
    771.2 |##
    783.0 |
    794.8 |########################################
    806.6 |
    818.5 |
    830.3 |
    842.1 |
    853.9 |
    865.7 |
    877.5 |
    889.3 |##
    901.1 |
  (3 below, 6 above range)

quantiser-software (n=40, range 3725.3-11066.1 ns)
   3725.3 |########################################
   4092.3 |
   4459.4 |
   4826.4 |
   5193.4 |
   5560.5 |
   5927.5 |
   6294.6 |##
   6661.6 |
   7028.7 |
   7395.7 |
   7762.8 |
   8129.8 |
   8496.8 |
   8863.9 |
   9230.9 |
   9598.0 |
   9965.0 |
  10332.1 |#
  10699.1 |#
  (3 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: bridge=14.9% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=61.4% (high variance, measurements may be unstable)
